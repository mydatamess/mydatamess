import { invoke } from "@tauri-apps/api/core";
import ResourceService, { GetRootResourceError } from "./ResourceService";
import { RootResource } from "../../models/resources";
import Result, { err, ok } from "../../models/results";
import z from "zod";
import { parseCommandError } from "../utils/tauri/error.utils";
import {
  GetRootResourceResponse,
  Operation,
  Request,
  Response,
} from "../../../generated/interface";

export class TauriResourceService implements ResourceService {
  async getRootResource(): Promise<Result<RootResource, GetRootResourceError>> {
    const request = Request.encode({
      operation: Operation.GET_ROOT_RESOURCE,
      payload: new Uint8Array(),
    }).finish();
    // eslint-disable-next-line no-useless-catch
    try {
      const response = await invoke<number[]>("main_cmd", {
        request: Array.from(request),
      });
      const decodedResponse = Response.decode(new Uint8Array(response));
      const rootResource = GetRootResourceResponse.decode(
        decodedResponse.payload,
      );
      return ok(rootResource);
    } catch (e) {
      throw e;
      // return err(
      //   parseCommandError(
      //     e,
      //     GetRootResourceErrorSchema,
      //     "Failed to get root resource",
      //   ),
      // );
    }
  }

  async getResources(): Promise<string[]> {
    return invoke("__resources_get_resources");
  }
}
