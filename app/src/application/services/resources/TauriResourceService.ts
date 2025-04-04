import { invoke } from "@tauri-apps/api/core";
import ResourceService, { GetRootResourceError } from "./ResourceService";
import { RootResource } from "../../models/resources";
import Result, { err, ok } from "../../models/results";
import z from "zod";
import { parseCommandError } from "../utils/tauri/error.utils";

export class TauriResourceService implements ResourceService {
  async getRootResource(): Promise<Result<RootResource, GetRootResourceError>> {
    try {
      const rootResource = await invoke("__resources_get_root_resource");
      return ok(RootResourceSchema.parse(rootResource));
    } catch (e) {
      return err(
        parseCommandError(
          e,
          GetRootResourceErrorSchema,
          "Failed to get root resource",
        ),
      );
    }
  }

  async getResources(): Promise<string[]> {
    return invoke("__resources_get_resources");
  }
}

const RootResourceSchema = z.object({
  id: z.string(),
  displayName: z.string(),
  description: z
    .string()
    .nullish()
    .transform((x) => x ?? undefined),
});

const GetRootResourceErrorSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("GenericError"),
    error: z.object({
      message: z.string(),
    }),
  }),
]);
