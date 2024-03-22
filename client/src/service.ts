export interface ListQuery {
  filter: string[],
  search: string,
  offset: number,
  length: number,
}
export function listQueryToParams(query: ListQuery): URLSearchParams {
  return new URLSearchParams({
    filter: query.filter.join(','),
    search: query.search,
    offset: query.offset.toString(),
    length: query.length.toString(),
  })
}

export interface ListResponse<T> {
  data: T[],
  count: number,
}
