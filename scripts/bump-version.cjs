#!/usr/bin/env node
/*
  Bumps version in:
  - package.json
  - src-tauri/tauri.conf.json
  - src-tauri/Cargo.toml (package version)

  Usage: node scripts/bump-version.cjs 0.4.2
  Accepts leading 'v' (v0.4.2) and strips it.
*/
const fs = require('fs');
const path = require('path');

function log(...args) {
  console.log('[bump-version]', ...args);
}

const rawArg = (process.argv[2] || process.env.VERSION || '').trim();
if (!rawArg) {
  console.error('Usage: node scripts/bump-version.cjs <version>');
  process.exit(1);
}
const version = rawArg.replace(/^v/, '');
const semverRe = /^(\d+)\.(\d+)\.(\d+)(?:[-+][0-9A-Za-z-.]+)?$/;
if (!semverRe.test(version)) {
  console.error(`Invalid version: ${rawArg}`);
  process.exit(1);
}

const root = process.cwd();
function readJson(p) {
  return JSON.parse(fs.readFileSync(p, 'utf8'));
}
function writeJson(p, obj) {
  fs.writeFileSync(p, JSON.stringify(obj, null, 2) + '\n');
}

// 1) package.json
const pkgPath = path.join(root, 'package.json');
const pkg = readJson(pkgPath);
pkg.version = version;
writeJson(pkgPath, pkg);
log('Updated package.json ->', version);

// 2) src-tauri/tauri.conf.json
const tauriConfPath = path.join(root, 'src-tauri', 'tauri.conf.json');
const tauriConf = readJson(tauriConfPath);
tauriConf.version = version;
writeJson(tauriConfPath, tauriConf);
log('Updated tauri.conf.json ->', version);

// 3) src-tauri/Cargo.toml (only first version occurrence)
const cargoPath = path.join(root, 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoPath, 'utf8');
// Replace version = "x.y.z" under [package] (first occurrence)
cargoToml = cargoToml.replace(/(^\s*version\s*=\s*")[^"]+("\s*$)/m, `$1${version}$2`);
fs.writeFileSync(cargoPath, cargoToml);
log('Updated Cargo.toml ->', version);

log('Done.');
