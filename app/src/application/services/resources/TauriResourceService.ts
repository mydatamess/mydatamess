import { invoke } from "@tauri-apps/api/core";
import Result, { err, ok } from "../../models/results";
import { GetRootResourceResponse } from "../../../generated/cmds/__resources_get_root_resource/response";
import { GetRootResourceRequest } from "../../../generated/cmds/__resources_get_root_resource/request";
import { GetRootResourceErrors } from "../../../generated/cmds/__resources_get_root_resource/error";

export class TauriResourceService {
  async getRootResource(): Promise<
    Result<GetRootResourceResponse, GetRootResourceErrors>
  > {
    try {
      const request: GetRootResourceRequest = {};
      const response = await invoke<GetRootResourceResponse>("cmd_gateway", {
        operationId: "__resources_get_root_resource",
        payload: request,
      });
      return ok(response);
    } catch (e) {
      return err(e as GetRootResourceErrors);
    }
  }
}
