import { validatePlugin } from "./validation.ts";
import { IPlugin, PluginConst, PluginInput, PluginManagerOptions, PluginMetadata } from "./types.ts";

const pluginNames = [];
let initialized = false;

function getMetadata(plugin) {
  if (typeof plugin === "function") {
    const proto = plugin.prototype;
    if (proto && typeof proto === "object" && "metadata" in proto) {
      return proto.metadata;
    }
    const instance = new plugin();
    return instance.metadata;
  }
  return plugin.metadata;
}

export function register(plugin) {
  validatePlugin(plugin);
  const metadata = getMetadata(plugin);
  pluginNames.push(metadata.name);
}

export function init() {
  if (initialized) return;
  initialized = true;
  // Hooks are called during register() for each plugin
}

export function shutdown() {
  pluginNames.length = 0;
  initialized = false;
}

export function getPlugins() {
  return [...pluginNames];
}

export function has(name) {
  for (let i = 0; i < pluginNames.length; i++) {
    if (pluginNames[i] === name) return true;
  }
  return false;
}
