export interface Tag {
  id: number;
  body: string | null | Tag[];
  argument: number | string | null;
}

export type Root = Tag[];

export class ParseError extends Error {
  constructor(message: string = "Parse error") {
    super(message);
    this.name = "ParseError";
  }
}
