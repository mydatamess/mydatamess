import z, { ZodSchema } from "zod";
import { GenericError, genericError } from "../../common/errors";

const CommandErrorSchema = z.object({
  type: z.string(),
  error: z.record(z.any()),
});

export type CommandError = z.infer<typeof CommandErrorSchema>;

/**
 * A generic function to handle command errors by validating them with a provided Zod schema.
 * Logs the error and returns a generic error response if validation fails.
 *
 * @param e - The caught error, which could be any type.
 * @param schema - The Zod schema used to validate the error.
 * @param message - A custom error message for logging and generating a generic error response.
 * @returns The validated error if the validation succeeds, or a generic error if validation fails.
 */
export function parseCommandError<T extends ZodSchema>(
  e: unknown,
  schema: T,
  message: string,
): z.infer<typeof schema> | GenericError {
  const error = CommandErrorSchema.safeParse(e);
  const parsedError = error.success
    ? schema.safeParse(error.data.error)
    : undefined;

  if (!parsedError?.success) {
    console.error(message, e);
    return genericError(message);
  }

  return parsedError.data;
}
