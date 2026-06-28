import { register, init, shutdown, getPlugins } from "./plugin_manager.ts";
import { PluginInput } from "./types.ts";

// --- Example: Register a plugin from a plain object ---
const helloPlugin: PluginInput = {
  metadata: { name: "hello", version: "1.0.0" },
  setup() {
    console.log("[hello] setup called");
  },
  onLoad() {
    console.log("[hello] loaded");
  },
  onEnable() {
    console.log("[hello] enabled");
  },
  onDisable() {
    console.log("[hello] disabled");
  },
  onUnload() {
    console.log("[hello] unloaded");
  },
};

// Register the plugin
register(helloPlugin);
console.log("Registered plugins:", getPlugins());

// Initialize (calls setup + onLoad for each plugin)
init();

// Shutdown (calls onDisable + onUnload for each plugin)
shutdown();
console.log("All plugins shut down.");
