type Result<T, E> =
  | { ok: true; value: T } // Success case
  | { ok: false; error: E }; // Error case

export default Result;

export function ok<T>(value: T): Result<T, never> {
  return { ok: true, value };
}

export function err<E>(error: E): Result<never, E> {
  return { ok: false, error };
}
