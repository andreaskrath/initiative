import * as z from "zod";

export type FieldErrors = Map<string, string>;

export const CreateFieldErrors = (errors: z.ZodError): FieldErrors => {
  const fieldErrors = new Map<string, string>();

  for (const issue of errors.issues) {
    const field = issue.path.join(".");
    fieldErrors.set(field, issue.message);
  }

  return fieldErrors;
};

/**
 * Check if a ZodError object contains any errors for a given field.
 *
 * Paths with more than one element, such as arrays, will be joined with a `.`,
 * and as such, the field should be `object.<index>.nested` as the field name.
 */
export const HasError = (errors: z.ZodError | null, field: string): boolean => {
  return (
    errors?.issues.some((issue) => issue.path.join(".") === field) ?? false
  );
};
