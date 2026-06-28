import { IPlugin, PluginInput, PluginMetadata } from "./types.ts";

export class PluginValidationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "PluginValidationError";
  }
}

function isValidMetadata(value) {
  if (!value || typeof value !== "object") return false;
  if (!value.name || typeof value.name !== "string") return false;
  if (value.name.length === 0) return false;
  if (!value.version || typeof value.version !== "string") return false;
  if (value.version.length === 0) return false;
  return true;
}

function validateHooks(name, obj) {
  const hooks = ["setup", "onLoad", "onEnable", "onDisable", "onUnload"];
  for (let i = 0; i < hooks.length; i++) {
    const hook = hooks[i];
    if (hook in obj && typeof obj[hook] !== "function") {
      throw new PluginValidationError(`Plugin "${name}": "${hook}" must be a function`);
    }
  }
}

function validateMetadata(obj) {
  if (!obj.metadata || typeof obj.metadata !== "object") {
    throw new PluginValidationError("Plugin must have a metadata object");
  }
  if (!isValidMetadata(obj.metadata)) {
    throw new PluginValidationError("Plugin metadata must have non-empty 'name' and 'version' strings");
  }
}

export function validatePlugin(plugin) {
  if (typeof plugin === "function") {
    const proto = plugin.prototype;
    if (proto && typeof proto === "object" && "metadata" in proto) {
      validateMetadata(proto);
      return;
    }
    try {
      const instance = new plugin();
      if (typeof instance === "object" && instance !== null && "metadata" in instance) {
        validateMetadata(instance);
        return;
      }
    } catch (e) {
      // instantiation failed
    }
    throw new PluginValidationError("Class plugin must have metadata in prototype or instance");
  }

  if (typeof plugin === "object" && plugin !== null) {
    validateMetadata(plugin);
    return;
  }

  throw new PluginValidationError("Plugin must be a class instance or a plain object with metadata");
}
