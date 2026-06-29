export interface PluginMetadata {
  name: string;
  version: string;
}

export interface IPlugin {
  readonly metadata: PluginMetadata;
  setup?(): void | Promise<void>;
  onLoad?(): void | Promise<void>;
  onEnable?(): void | Promise<void>;
  onDisable?(): void | Promise<void>;
  onUnload?(): void | Promise<void>;
}

export type PluginConst = {
  metadata: PluginMetadata;
  setup?: () => void | Promise<void>;
  onLoad?: () => void | Promise<void>;
  onEnable?: () => void | Promise<void>;
  onDisable?: () => void | Promise<void>;
  onUnload?: () => void | Promise<void>;
};

export type PluginClass = new () => IPlugin;

export type PluginInput = IPlugin | PluginConst | PluginClass;

export type PluginManagerOptions = Record<string, unknown>;
