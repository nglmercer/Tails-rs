export interface PluginMetadata {
  name: string;
  version: string;
}

export interface IPlugin {
  metadata: PluginMetadata;
  setup: () => void;
  onLoad: () => void;
  onEnable: () => void;
  onDisable: () => void;
  onUnload: () => void;
}

export interface PluginConst {
  metadata: PluginMetadata;
  setup?: () => void;
  onLoad?: () => void;
  onEnable?: () => void;
  onDisable?: () => void;
  onUnload?: () => void;
}

export type PluginInput = IPlugin | PluginConst;

export type PluginManagerOptions = Record<string, unknown>;
