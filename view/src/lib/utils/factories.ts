/**
 * Generic factory helper for creating Records.
 */
export function RecordFactory<T extends string | number | symbol, V>(
  items: readonly T[],
  defaultValue: V,
): Record<T, V> {
  return items.reduce(
    (record, item) => {
      record[item] = defaultValue;
      return record;
    },
    {} as Record<T, V>,
  );
}
