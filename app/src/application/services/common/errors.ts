export type GenericError = {
  type: "GenericError";
  error: { message?: string };
};

export const genericError = (message?: string): GenericError => {
  return {
    type: "GenericError",
    error: { message },
  };
};
