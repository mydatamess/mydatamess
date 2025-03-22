import { invoke } from "@tauri-apps/api/core";
import ResourceService from "./ResourceService";

export class TauriResourceService implements ResourceService {
  async getResources(): Promise<string[]> {
    return invoke("__resources_get_resources");
  }
}
