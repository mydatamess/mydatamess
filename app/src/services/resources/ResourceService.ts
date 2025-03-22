export default interface ResourceService {
  getResources(): Promise<string[]>;
}
