import { ResourceGroupIdentifier } from "./resource_groups";

export type ResourceIdentifier = string;

export type ResourceBase = {
  id: ResourceIdentifier;
  displayName: string;
  description?: string;
};

export type RootResource = ResourceBase & {};

export type ChildResource = ResourceBase & {
  groupId: ResourceGroupIdentifier;
};
