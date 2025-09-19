// Copy the built Tauri executable to installer/payload/build
// Assumes windows build. Adjust pattern for other platforms if needed.
const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..', '..'); // legacy monorepo root (may not exist after split)
const tauriDir = path.resolve(__dirname, '..');   // launcher-tauri root (or new repo root)

// Tauri output (release) default path
const releaseDir = path.join(tauriDir, 'src-tauri', 'target', 'release');

// Destination inside installer/payload/build
// Destination (legacy path). If missing, skip gracefully in standalone repo mode.
const destDir = path.join(root, 'installer', 'payload', 'build');

if (!fs.existsSync(releaseDir)) {
  console.warn('[copy-exe] Release directory not found, skipping:', releaseDir);
  process.exit(0);
}

if (!fs.existsSync(path.join(root, 'installer'))) {
  console.warn('[copy-exe] Legacy installer path not found, skipping copy (standalone repo?):', path.join(root, 'installer'));
  process.exit(0);
}

if (!fs.existsSync(destDir)) {
  fs.mkdirSync(destDir, { recursive: true });
}

// Find the first .exe that matches product name or generic pattern
const exeCandidates = fs.readdirSync(releaseDir).filter((f) => f.toLowerCase().endsWith('.exe'));
if (exeCandidates.length === 0) {
  console.error('[copy-exe] Aucune executable trouvée dans', releaseDir);
  process.exit(1);
}

// Heuristic: prefer one containing 'launcher'
let exeName = exeCandidates.find((f) => f.toLowerCase().includes('launcher')) || exeCandidates[0];

const src = path.join(releaseDir, exeName);
const dest = path.join(destDir, 'AvrixLauncher.exe');

try {
  fs.copyFileSync(src, dest);
  console.log(`[copy-exe] Copié ${src} -> ${dest}`);
} catch (e) {
  console.error('[copy-exe] Erreur copie:', e);
  process.exit(1);
}
