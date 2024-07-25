import { encode } from "@msgpack/msgpack";
import { bodyToRaw } from "../utils";

export type RawBody = string | null | RawTag[] | RawTag;
export type RawId = number;
export type RawArgument = number | string | null;
export type RawTagAsArray = [RawId, RawBody, RawArgument] | [RawId, RawBody];

export type RawTag = RawTagAsArray | number | string | RawTag[];
export type RootRaw = RawTag[];

export class ParseError extends Error {
  constructor(message: string = "Parse error") {
    super(message);
    this.name = "ParseError";
  }
}

export abstract class Tag {
  id: number;
  body: Body;
  argument: Argument;
  constructor(id: number, body: Body, argument: Argument) {
    this.id = id;
    this.body = body;
    this.argument = argument;
  }
  get raw(): RawTag {
    if (this.argument == null) {
      if (this.body == null) {
        return this.id;
      }

      return [this.id, bodyToRaw(this.body)];
    }
    return [this.id, bodyToRaw(this.body), this.argument];
  }
  encode(): Uint8Array {
    return encode(this.raw);
  }
  abstract toHtml(classes?: boolean): string;
}

export type Body = string | Tag[] | null;
export type Argument = RawArgument;

export type TagNormalizer = (tag: RawTagAsArray) => Tag;
