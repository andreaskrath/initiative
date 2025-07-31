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

export const LabelValueFactory = <T extends string>(
  items: readonly T[],
): Array<{ value: T; label: T }> =>
  items.map((item) => ({
    value: item,
    label: item,
  }));
