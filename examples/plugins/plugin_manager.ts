import {
  IPlugin,
  PluginClass,
  PluginConst,
  PluginEntry,
  PluginHook,
  PluginInput,
  PluginMetadata,
} from "./types.ts";
import { validatePlugin } from "./validation.ts";

const plugins: PluginEntry[] = [];
let initialized: boolean = false;

function isClassPlugin(plugin: PluginInput): plugin is PluginClass {
  return typeof plugin === "function" && plugin.prototype !== undefined;
}

function isObjectPlugin(plugin: PluginInput): plugin is IPlugin | PluginConst {
  return typeof plugin === "object" && plugin !== null && "metadata" in plugin;
}

function getMetadata(plugin: PluginInput): PluginMetadata {
  if (isClassPlugin(plugin)) {
    const proto = plugin.prototype;
    if (proto && typeof proto === "object" && "metadata" in proto) {
      return proto.metadata as PluginMetadata;
    }
    const instance = new plugin();
    return instance.metadata;
  }
  if (isObjectPlugin(plugin)) {
    return plugin.metadata;
  }
  throw new Error("Invalid plugin input");
}

function getHookFn(
  plugin: PluginInput,
  hook: PluginHook,
): (() => void | Promise<void>) | null {
  if (isClassPlugin(plugin)) {
    const proto = plugin.prototype;
    if (proto && typeof proto === "object" && hook in proto) {
      const fn = proto[hook];
      if (typeof fn === "function") {
        return fn.bind(proto);
      }
    }
    return null;
  }

  if (isObjectPlugin(plugin)) {
    const obj = plugin as Record<string, unknown>;
    if (hook in obj && typeof obj[hook] === "function") {
      return (obj[hook] as () => void | Promise<void>).bind(plugin);
    }
  }

  return null;
}

export function register(plugin: PluginInput): void {
  validatePlugin(plugin);
  const metadata = getMetadata(plugin);

  if (has(metadata.name)) {
    throw new Error(`Plugin "${metadata.name}" is already registered`);
  }

  const entry: PluginEntry = {
    name: metadata.name,
    instance: null,
    hooks: {
      setup: getHookFn(plugin, "setup"),
      onLoad: getHookFn(plugin, "onLoad"),
      onEnable: getHookFn(plugin, "onEnable"),
      onDisable: getHookFn(plugin, "onDisable"),
      onUnload: getHookFn(plugin, "onUnload"),
    },
  };

  if (isObjectPlugin(plugin)) {
    entry.instance = plugin as IPlugin;
  }

  plugins.push(entry);
}

export async function init(): Promise<void> {
  if (initialized) return;
  initialized = true;

  for (const plugin of plugins) {
    if (plugin.hooks.setup) {
      await plugin.hooks.setup();
    }
    if (plugin.hooks.onLoad) {
      await plugin.hooks.onLoad();
    }
    if (plugin.hooks.onEnable) {
      await plugin.hooks.onEnable();
    }
  }
}

export async function shutdown(): Promise<void> {
  for (const plugin of plugins) {
    if (plugin.hooks.onDisable) {
      await plugin.hooks.onDisable();
    }
    if (plugin.hooks.onUnload) {
      await plugin.hooks.onUnload();
    }
  }
  plugins.length = 0;
  initialized = false;
}

export function getPlugins(): string[] {
  return plugins.map((p) => p.name);
}

export function has(name: string): boolean {
  return plugins.some((p) => p.name === name);
}

export function getEntry(name: string): PluginEntry | undefined {
  return plugins.find((p) => p.name === name);
}

export function isInitialized(): boolean {
  return initialized;
}

export async function enablePlugin(name: string): Promise<boolean> {
  const entry = getEntry(name);
  if (!entry) return false;

  if (entry.hooks.onEnable) {
    await entry.hooks.onEnable();
  }
  return true;
}

export async function disablePlugin(name: string): Promise<boolean> {
  const entry = getEntry(name);
  if (!entry) return false;

  if (entry.hooks.onDisable) {
    await entry.hooks.onDisable();
  }
  return true;
}
