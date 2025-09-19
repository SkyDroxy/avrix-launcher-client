export interface PluginInfo {
  name: string;
  sizeKB: number;
  modified: number;
  version?: string;
  displayName?: string;
  environment?: string;
  author?: string;
  license?: string;
  id?: string;
  description?: string;
  dependencies?: Record<string, string>;
  image?: string;
  imageUrl?: string;
  internal?: boolean;
  parentId?: string;
}

export interface ScanPluginsResult {
  plugins?: PluginInfo[];
  dir?: string;
}

export {};
