import { encodeTag } from "../daletpack";
import { bodyToRaw } from "../utils";

export type Root = Tag[];
export type Argument = string | number | null;
export type Body = string | Tag[] | null;

export interface Tag {
  id: number;
  body: Body;
  argument: Argument;
}

export class ParseError extends Error {
  constructor(message: string = "Parse error") {
    super(message);
    this.name = "ParseError";
  }
}

export abstract class CommonTag {
  id: number;
  body: CommonBody;
  argument: Argument;
  constructor(id: number, body: CommonBody, argument: Argument) {
    this.id = id;
    this.body = body;
    this.argument = argument;
  }
  get raw(): Tag {
    return {
      id: this.id,
      body: bodyToRaw(this.body),
      argument: this.argument,
    };
  }
  encode(): Uint8Array {
    return encodeTag(this.raw);
  }
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  toHtml(classes?: boolean): string {
    return "";
  }
}

export type CommonBody = CommonTag[] | null;

export type TagNormalizer = (tag: Tag) => CommonTag;
