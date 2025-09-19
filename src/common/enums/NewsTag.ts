// Canonical NewsTag enum
export enum NewsTag {
  PATCH = 'Patch',
  EVENT = 'Event',
  UPDATE = 'Update',
  FEATURE = 'Feature',
}

export type NewsTagValue = `${NewsTag}`;
export const NewsTagValues: ReadonlySet<NewsTagValue> = new Set(
  Object.values(NewsTag)
) as ReadonlySet<NewsTagValue>;
export function isNewsTagValue(v: string): v is NewsTagValue {
  return (NewsTagValues as Set<string>).has(v);
}
