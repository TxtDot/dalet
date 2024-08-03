export type Page = Tag[];
export type Body = string | Tag[] | null;
export type Argument = string | number | null;

export interface Tag {
  id: number;
  body: Body;
  argument: Argument;
}
