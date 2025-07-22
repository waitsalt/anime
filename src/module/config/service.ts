import { invoke } from "@tauri-apps/api/core";
import { Config } from "./model";

async function configGet() {
  const res: Config = await invoke("config_get");
  return res;
}

async function configSet(config: Config) {
  await invoke("config_set", { config });
  return null;
}

export { configGet, configSet };
