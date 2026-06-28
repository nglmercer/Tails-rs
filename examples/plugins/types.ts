export interface PluginMetadata {
  readonly name: string;
  readonly version: string;
}

export interface IPlugin {
  readonly metadata: PluginMetadata;
  setup(): void | Promise<void>;
  onLoad(): void | Promise<void>;
  onEnable(): void | Promise<void>;
  onDisable(): void | Promise<void>;
  onUnload(): void | Promise<void>;
}

export interface PluginConst {
  metadata: PluginMetadata;
  setup?(): void | Promise<void>;
  onLoad?(): void | Promise<void>;
  onEnable?(): void | Promise<void>;
  onDisable?(): void | Promise<void>;
  onUnload?(): void | Promise<void>;
}

export type PluginInput = IPlugin | PluginConst;

export type PluginManagerOptions = Record<string, unknown>;
