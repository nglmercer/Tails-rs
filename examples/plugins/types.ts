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

export type PluginClass = new (...args: unknown[]) => IPlugin;

export type PluginInput = IPlugin | PluginConst | PluginClass;

export type PluginManagerOptions = Record<string, unknown>;

export type PluginHook = "setup" | "onLoad" | "onEnable" | "onDisable" | "onUnload";

export interface PluginEntry {
  name: string;
  instance: IPlugin | null;
  hooks: Record<PluginHook, (() => void | Promise<void>) | null>;
}
