import {
  register,
  init,
  shutdown,
  getPlugins,
} from "./plugin_manager.ts";
import { PluginInput } from "./types.ts";

const helloPlugin: PluginInput = {
  metadata: { name: "hello", version: "1.0.0" },
  setup(): void {
    console.log("[hello] setup called");
  },
  onLoad(): void {
    console.log("[hello] loaded");
  },
  onEnable(): void {
    console.log("[hello] enabled");
  },
  onDisable(): void {
    console.log("[hello] disabled");
  },
  onUnload(): void {
    console.log("[hello] unloaded");
  },
};

async function main(): Promise<void> {
  register(helloPlugin);
  console.log("Registered plugins:", getPlugins());

  await init();
  console.log("Initialized.");

  await shutdown();
  console.log("All plugins shut down.");
}

main();
