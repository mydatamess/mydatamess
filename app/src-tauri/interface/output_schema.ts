class Client {
  async ping(req: PingRequest): Promise<PingResponse> {
    try {
      return await invoke<PingResponse>("ping", req);
    } catch (e) {
      throw e as PingError;
    }
  }

}
