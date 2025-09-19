// Helper functions related to plugin meta formatting & fallbacks
import type { PluginInfo } from '@interfaces/plugins';

const filenameVersionRegex = /(.+?)[-_]v?(\d+\.\d+(?:\.\d+)?)(?:[-_].*)?$/i;

export function parseFromFilename(name: string) {
  const base = name.replace(/\.jar$/i, '');
  const m = base.match(filenameVersionRegex);
  if (m) return { display: m[1], version: m[2] };
  return { display: base, version: 'Inconnu' };
}

export function effectiveDisplayName(p: PluginInfo) {
  return p.displayName || parseFromFilename(p.name).display;
}

export function effectiveVersion(p: PluginInfo) {
  return p.version || parseFromFilename(p.name).version;
}

export function environmentBadge(p: PluginInfo) {
  return p.environment?.toUpperCase();
}

export function displayName(p: PluginInfo) {
  return effectiveDisplayName(p);
}
export function versionOf(p: PluginInfo) {
  return effectiveVersion(p);
}
export function hasDeps(p: PluginInfo) {
  return !!(p.dependencies && Object.keys(p.dependencies).length > 0);
}
export function sortedDeps(p: PluginInfo): [string, string][] {
  if (!p.dependencies) return [];
  return Object.entries(p.dependencies).sort((a, b) => a[0].localeCompare(b[0]));
}
export function keyFor(p: PluginInfo) {
  return p.id || slugFromName(p.name);
}
export function slugFromName(name: string) {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, '-');
}

// Formatting helpers centralized
export function formatSize(sizeKB: number | undefined) {
  if (!sizeKB || Number.isNaN(sizeKB)) return '—';
  if (sizeKB < 1024) return sizeKB.toFixed(0) + ' KB';
  const mb = sizeKB / 1024;
  return (mb >= 100 ? mb.toFixed(0) : mb.toFixed(1)) + ' MB';
}
export function fullDate(ts?: number) {
  if (!ts) return '';
  const d = new Date(ts * 1000);
  return d.toLocaleDateString() + ' ' + d.toLocaleTimeString();
}
export function shortDate(ts?: number) {
  if (!ts) return '';
  const d = new Date(ts * 1000);
  return d.toLocaleDateString();
}
