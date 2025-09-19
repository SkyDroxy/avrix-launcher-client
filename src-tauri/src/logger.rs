use once_cell::sync::Lazy;
use serde::Serialize;
use tauri::Emitter;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    panic,
    path::PathBuf,
    sync::{Mutex, Once},
};

static LOG_FILE: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
static CRASH_FILE: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
static INIT_HOOK: Once = Once::new();

#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Warn,
    Error,
}
impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

fn logs_dir() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let dir = exe_dir.join("logs");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn ensure_log_path() -> PathBuf {
    let mut guard = LOG_FILE.lock().unwrap();
    if let Some(p) = guard.as_ref() {
        return p.clone();
    }
    let file = logs_dir().join("avrix-launcher.log");
    *guard = Some(file.clone());
    file
}

fn ensure_crash_path() -> PathBuf {
    let mut guard = CRASH_FILE.lock().unwrap();
    if let Some(p) = guard.as_ref() {
        return p.clone();
    }
    let file = logs_dir().join("avrix-launcher.crash.log");
    *guard = Some(file.clone());
    file
}

fn write_line(path: &PathBuf, line: &str) {
    if path.ends_with("avrix-launcher.log") {
        if let Ok(meta) = fs::metadata(&path) {
            if meta.len() > 2 * 1024 * 1024 {
                let _ = fs::remove_file(path.with_extension("log.1"));
                let _ = fs::rename(&path, path.with_extension("log.1"));
            }
        }
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{}", line);
    }
}

pub fn log(level: Level, source: &str, message: &str) {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let sanitized = message.replace('\n', "\\n");
    let line = format!("{} [{}][{}] {}", ts, level.as_str(), source, sanitized);
    let path = ensure_log_path();
    write_line(&path, &line);
}

pub fn info(source: &str, message: &str) {
    log(Level::Info, source, message);
}
pub fn warn(source: &str, message: &str) {
    log(Level::Warn, source, message);
}
pub fn error(source: &str, message: &str) {
    log(Level::Error, source, message);
}

#[derive(Serialize, Clone)]
struct AppLog<'a> {
    ts: String,
    level: &'a str,
    source: &'a str,
    message: String,
}

pub fn emit_app_log(window: &tauri::Window, level: Level, source: &str, message: &str) -> tauri::Result<()> {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let payload = AppLog {
        ts,
        level: level.as_str(),
        source,
        message: message.to_string(),
    };
    window.emit("app-log", payload)
}

pub fn setup_global_handlers(app: &tauri::AppHandle) {
    let handle = app.clone();
    INIT_HOOK.call_once(|| {
        panic::set_hook(Box::new(move |panic_info| {
            let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut reason = String::new();
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                reason = s.to_string();
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                reason = s.clone();
            } else {
                reason = "unknown panic".to_string();
            }
            let location = if let Some(loc) = panic_info.location() {
                format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
            } else {
                "<unknown>".to_string()
            };
            let bt = std::backtrace::Backtrace::force_capture();
            let path = ensure_crash_path();
            let header = format!(
                "{} [CRASH] reason={} location={}",
                ts, reason.replace('\n', "\\n"), location
            );
            write_line(&path, &header);
            write_line(&path, &format!("backtrace:\n{:?}", bt));
            // Also mirror to main log as ERROR
            error("panic", &format!("{} | {}", reason, location));
            let _ = handle.emit("app-log", AppLog {
                ts: ts.to_string(),
                level: "ERROR",
                source: "panic",
                message: format!("{} @ {}", reason, location),
            });
        }));
    });
}