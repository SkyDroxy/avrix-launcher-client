export interface PluginInfo {
  name: string;
  sizeKB: number;
  modified: number;
  version?: string; // parsed from metadata or filename
  displayName?: string; // name without .jar + version suffix
  environment?: string;
  author?: string;
  license?: string;
  id?: string;
  description?: string;
  dependencies?: Record<string, string>; // map name -> version constraint
  image?: string; // embedded image path or resolved base64
  imageUrl?: string; // remote fallback
  internal?: boolean; // bundled logical plugin (no standalone jar)
  parentId?: string; // id of parent (e.g. avrix-core)
}

export interface ScanPluginsResult {
  plugins?: PluginInfo[];
  dir?: string;
}

// ensure this file is treated as a module (helps some tooling when only interfaces are present)
export {};
