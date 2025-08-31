import * as z from "zod";

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
