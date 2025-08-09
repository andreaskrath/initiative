/**
 * Generic factory helper for creating Records.
 */
export const RecordFactory = <T extends string | number | symbol, V>(
  items: readonly T[],
  defaultValue: V,
): Record<T, V> =>
  items.reduce(
    (record, item) => {
      record[item] = defaultValue;
      return record;
    },
    {} as Record<T, V>,
  );

/**
 * Helper factory for converting a list of string-like values to label-value objects.
 */
export const ToLabelValue = <T extends string>(
  items: readonly T[],
): Array<{ value: T; label: T }> =>
  items.map((item) => ({
    value: item,
    label: item,
  }));

/**
 * Helper factory for converting a list of string-like values to label-value objects, via the provided record.
 */
export const ToLabelValueWith = <T extends string>(
  items: readonly T[],
  labels: Record<T, string>,
): Array<{ value: T; label: string }> =>
  items.map((item) => ({
    value: item,
    label: labels[item],
  }));
