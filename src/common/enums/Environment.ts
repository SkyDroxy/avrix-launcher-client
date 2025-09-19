// Canonical Environment enum for plugin filtering and metadata.
// Values are lowercase to match backend provided metadata (Rust scanner returns lowercase strings).
export enum Environment {
  CLIENT = 'client',
  SERVER = 'server',
  BOTH = 'both',
}

export type EnvironmentValue = `${Environment}`;
export const EnvironmentValues: ReadonlySet<EnvironmentValue> = new Set(
  Object.values(Environment)
) as ReadonlySet<EnvironmentValue>;

export function isEnvironmentValue(v: string): v is EnvironmentValue {
  return (EnvironmentValues as Set<string>).has(v.toLowerCase());
}

export const EnvironmentLabels: Record<EnvironmentValue, string> = {
  [Environment.CLIENT]: 'Client',
  [Environment.SERVER]: 'Server',
  [Environment.BOTH]: 'Both',
};
