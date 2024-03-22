export interface Meta {
  path: string;
  created_at: string;
  updated_at: string;
}

export enum MetaCmpBy {
  Default = "default",
  Path = "path",
  CreatedAt = "created_at",
  UpdatedAt = "updated_at",
}
