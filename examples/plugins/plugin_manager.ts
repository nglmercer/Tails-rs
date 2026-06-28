import {
  IPlugin,
  PluginConst,
  PluginInput,
  PluginManagerOptions,
  PluginMetadata,
} from "./types.ts";
import { validatePlugin } from "./validation.ts";

const pluginNames: string[] = [];
const pluginSetups: Array<(() => void) | null> = [];
const pluginOnLoads: Array<(() => void) | null> = [];
const pluginOnDisables: Array<(() => void) | null> = [];
const pluginOnUnloads: Array<(() => void) | null> = [];
let initialized: boolean = false;

function getMetadata(plugin: PluginInput): PluginMetadata {
  if (typeof plugin === "function") {
    const proto = (plugin as { prototype?: Record<string, unknown> }).prototype;
    if (proto && typeof proto === "object" && "metadata" in proto) {
      return proto.metadata as PluginMetadata;
    }
    const instance = new (plugin as new () => IPlugin)();
    return instance.metadata;
  }
  return plugin.metadata;
}

export function register(plugin: PluginInput): void {
  validatePlugin(plugin);
  const metadata: PluginMetadata = getMetadata(plugin);
  pluginNames.push(metadata.name);
  pluginSetups.push(
    typeof plugin.setup === "function" ? plugin.setup.bind(plugin) : null,
  );
  pluginOnLoads.push(
    typeof plugin.onLoad === "function" ? plugin.onLoad.bind(plugin) : null,
  );
  pluginOnDisables.push(
    typeof plugin.onDisable === "function"
      ? plugin.onDisable.bind(plugin)
      : null,
  );
  pluginOnUnloads.push(
    typeof plugin.onUnload === "function"
      ? plugin.onUnload.bind(plugin)
      : null,
  );
}

export function init(): void {
  if (initialized) return;
  initialized = true;
  for (let i = 0; i < pluginNames.length; i++) {
    if (pluginSetups[i]) pluginSetups[i]();
    if (pluginOnLoads[i]) pluginOnLoads[i]();
  }
}

export function shutdown(): void {
  for (let i = 0; i < pluginNames.length; i++) {
    if (pluginOnDisables[i]) pluginOnDisables[i]();
    if (pluginOnUnloads[i]) pluginOnUnloads[i]();
  }
  pluginNames.length = 0;
  pluginSetups.length = 0;
  pluginOnLoads.length = 0;
  pluginOnDisables.length = 0;
  pluginOnUnloads.length = 0;
  initialized = false;
}

export function getPlugins(): string[] {
  return [...pluginNames];
}

export function has(name: string): boolean {
  for (let i = 0; i < pluginNames.length; i++) {
    if (pluginNames[i] === name) return true;
  }
  return false;
}
