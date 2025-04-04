import { ResourceIdentifier } from "./resources";

export type ResourceGroupIdentifier = {
  parentResourceId: ResourceIdentifier;
  key: string;
};

export type ResourceGroup = {
  id: ResourceGroupIdentifier;
  displayName: string;
  description?: string;
  priority?: number;
};
