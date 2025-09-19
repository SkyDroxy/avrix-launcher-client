use crate::logger::{emit_app_log, error, info, Level};
use crate::util::find_game_root;
use anyhow::{anyhow, Result};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};
use tauri::{Emitter, Window};

pub fn launch_game(window: Window, steam: bool, mem_mb: Option<u64>) -> Result<String> {
    info("launch", &format!("launch_game invoked (steam={}, mem_mb={:?})", steam, mem_mb));
    let ctx = resolve_launch_context()?;
    let core_jar = ctx
        .core_jar
        .ok_or_else(|| anyhow!("[Error] Avrix-Core.jar not found."))?;
    let emit = |lvl: Level, m: &str| {
        match lvl {
            Level::Info => info("launch", m),
            Level::Warn => crate::logger::warn("launch", m),
            Level::Error => error("launch", m),
        }
        let _ = window.emit("launch-log", m.to_string());
        let _ = emit_app_log(&window, lvl, "launch", m);
    };
    emit(Level::Info, &format!("[JarLookup] Jar chosen : {}", core_jar.display()));
    emit(Level::Info, &format!("[Classpath] {}", ctx.class_path));
    emit(Level::Info, &format!("[LibPath] {}", ctx.library_path));

    let is_64 = cfg!(target_pointer_width = "64");
    let mut args: Vec<String> = vec![
        "-Djdk.attach.allowAttachSelf=true".into(),
        "-XX:+EnableDynamicAgentLoading".into(),
        "-Djava.awt.headless=true".into(),
        "-Davrix.mode=client".into(),
        format!("-Dzomboid.steam={}", if steam { 1 } else { 0 }),
        "-Dzomboid.znetlog=1".into(),
        format!("-Djava.library.path={}", ctx.library_path),
    ];
    let target_mb: u64 = mem_mb.unwrap_or_else(|| if is_64 { 3072 } else { 1200 });
    let target_mb = target_mb.max(256);
    if is_64 {
        args.push("-XX:+UseZGC".into());
    } else {
        args.push("-XX:+UseG1GC".into());
    }
    let xms_mb = std::cmp::max(256, std::cmp::min(target_mb / 2, target_mb));
    args.push(format!("-Xms{}m", xms_mb));
    args.push(format!("-Xmx{}m", target_mb));
    args.push("-cp".into());
    args.push(ctx.class_path.clone());
    args.push("com.avrix.Launcher".into());
    emit(Level::Info, &format!("Java used  : {}", ctx.java_path.display()));
    emit(
        Level::Info,
        &format!("Command: {} {}", ctx.java_path.display(), args.join(" ")),
    );

    let mut command = Command::new(&ctx.java_path);
    command
        .args(&args)
        .current_dir(&ctx.work_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }

    let mut child = command.spawn()?;
    emit(Level::Info, "Process spawned successfully");
    let window_out = window.clone();
    if let Some(out) = child.stdout.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                let _ = window_out.emit("launch-log", line);
            }
        });
    }
    let window_err = window.clone();
    if let Some(err) = child.stderr.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                let _ = window_err.emit("launch-log", line);
            }
        });
    }
    let window_exit = window.clone();
    std::thread::spawn(move || {
        if let Ok(status) = child.wait() {
            let code = status.code().unwrap_or(-1);
            let _ = window_exit.emit("launch-exit", code);
        }
    });
    Ok("[Launch in progress – live stream]".into())
}

struct LaunchContext {
    core_jar: Option<PathBuf>,
    work_dir: PathBuf,
    class_path: String,
    library_path: String,
    java_path: PathBuf,
}

fn resolve_launch_context() -> Result<LaunchContext> {
    let base = std::env::current_dir()?;
    let game_root = find_game_root(&base).unwrap_or(base.clone());
    let mut candidates: Vec<PathBuf> = vec![];
    let search_dirs = [
        base.clone(),
        game_root.clone(),
        base.join(".."),
        base.join("..").join(".."),
        base.join("libs"),
        base.join("bin"),
        base.join("core"),
        base.parent().unwrap_or(&base).to_path_buf(),
    ];
    for d in search_dirs.iter() {
        if d.exists() {
            for entry in fs::read_dir(d)? {
                if let Ok(e) = entry {
                    let p = e.path();
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("Avrix-Core") && name.ends_with(".jar") {
                            candidates.push(p);
                        }
                    }
                }
            }
        }
    }
    candidates.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
    let core_jar = candidates.last().cloned();
    let mut raw_entries: Vec<String> = Vec::new();
    raw_entries.push(game_root.to_string_lossy().to_string());
    raw_entries.push(base.to_string_lossy().to_string());
    for p in [&base, &game_root] {
        if p.exists() {
            for entry in fs::read_dir(p)? {
                if let Ok(e) = entry {
                    let path = e.path();
                    if let Some(n) = path.file_name().and_then(|n| n.to_str()) {
                        if n.to_lowercase().ends_with(".jar") {
                            raw_entries.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    let mut dedup: Vec<String> = Vec::new();
    for e in raw_entries.into_iter() {
        if seen.insert(e.clone()) {
            dedup.push(e);
        }
    }
    if let Some(j) = &core_jar {
        let cj = j.to_string_lossy().to_string();
        dedup.retain(|v| v != &cj);
        dedup.push(cj);
    }
    let class_path = dedup.join(if cfg!(windows) { ";" } else { ":" });
    let win_dir = if cfg!(target_pointer_width = "64") {
        "win64"
    } else {
        "win32"
    };
    let mut lib_parts: Vec<String> = vec![];
    for p in [&base, &game_root] {
        if p.exists() {
            let base_s = p.to_string_lossy().to_string();
            if !lib_parts.contains(&base_s) {
                lib_parts.push(base_s);
            }
            let w = p.join(win_dir);
            if w.exists() {
                let w_s = w.to_string_lossy().to_string();
                if !lib_parts.contains(&w_s) {
                    lib_parts.push(w_s);
                }
            }
        }
    }
    let library_path = lib_parts.join(if cfg!(windows) { ";" } else { ":" });
    let java_path = find_java()?;
    let work_dir = game_root.clone();
    Ok(LaunchContext {
        core_jar,
        work_dir,
        class_path,
        library_path,
        java_path,
    })
}

fn find_java() -> Result<PathBuf> {
    let exe_name = if cfg!(windows) { "javaw.exe" } else { "java" };
    let fallback_exe_name = if cfg!(windows) { "java.exe" } else { "java" };
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(cur) = std::env::current_dir() {
        candidates.push(cur.join("jre").join("bin").join(exe_name));
        candidates.push(cur.join("jre").join("bin").join(fallback_exe_name));
    }
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(base) = exe_path.parent() {
            candidates.push(base.join("jre").join("bin").join(exe_name));
            candidates.push(base.join("jre").join("bin").join(fallback_exe_name));
            if let Some(parent) = base.parent() {
                candidates.push(parent.join("jre").join("bin").join(exe_name));
                candidates.push(parent.join("jre").join("bin").join(fallback_exe_name));
            }
        }
    }
    if let Ok(home) = std::env::var("JAVA_HOME") {
        candidates.push(PathBuf::from(&home).join("bin").join(exe_name));
        candidates.push(PathBuf::from(&home).join("bin").join(fallback_exe_name));
    }
    for c in candidates {
        if c.exists() {
            return Ok(c);
        }
    }
    if let Ok(p) = which::which(exe_name) {
        return Ok(p);
    }
    if let Ok(p) = which::which(fallback_exe_name) {
        return Ok(p);
    }
    Err(anyhow!(
        "java not found (bundled ./jre, JAVA_HOME or PATH)"
    ))
}

