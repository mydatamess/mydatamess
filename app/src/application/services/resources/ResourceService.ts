import { RootResource } from "../../models/resources";
import Result from "../../models/results";
import { GenericError } from "../common/errors";

export default interface ResourceService {
  getRootResource(): Promise<Result<RootResource, GetRootResourceError>>;
  getResources(): Promise<string[]>;
}

export type GetRootResourceError = GenericError;
