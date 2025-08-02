export const PrepareForValidation = <T>(obj: T): T => {
  if (obj === null || obj === undefined) {
    return obj;
  }

  if (Array.isArray(obj)) {
    return obj.map((item) => PrepareForValidation(item)) as T;
  }

  if (typeof obj === "object" && obj.constructor === Object) {
    const cleaned: any = {};

    for (const [key, value] of Object.entries(obj)) {
      if (value === "" || value === null) {
        cleaned[key] = undefined;
      } else if (value !== null && typeof value === "object") {
        cleaned[key] = PrepareForValidation(value);
      } else {
        cleaned[key] = value;
      }
    }

    return cleaned as T;
  }

  return obj;
};
