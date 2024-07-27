import { encode, decode } from "@msgpack/msgpack";
import {
  ParseError,
  RawTag,
  Tag,
  RawTagAsArray,
  RawBody,
  Body,
  RootRaw,
} from "./types";
import El from "./tags/el";
import { TagNormalizers } from "./normalizers";

export function parseTag(raw_tag: RawTag): Tag {
  if (typeof raw_tag === "string") {
    return new El(raw_tag);
  }

  if (Array.isArray(raw_tag)) {
    if (Array.isArray(raw_tag[0])) {
      raw_tag = raw_tag as RawTag[];
      return new El(raw_tag.map(parseTag));
    }

    if (typeof raw_tag[0] === "number") {
      return TagNormalizers[(raw_tag as RawTagAsArray)[0]](
        raw_tag as RawTagAsArray
      );
    }
  }

  throw new ParseError("Invalid tag");
}

export function parseBody(body: RawBody): Body {
  if (typeof body === "string") {
    return body;
  }

  if (body === null) {
    return null;
  }

  if (Array.isArray(body)) {
    if (Array.isArray(body[0])) {
      return body.map((t) => parseTag(t as RawTag));
    }

    if (typeof body[0] === "number") {
      return [parseTag(body)];
    }
  }

  throw new ParseError("Invalid tag body");
}

export function parse(root_data: Uint8Array): Root {
  const root = decode(root_data);

  if (!Array.isArray(root)) {
    throw new ParseError("Daletl root must be array");
  }

  return new Root(root.map(parseTag));
}

export class Root {
  root: Tag[];
  constructor(root: Tag[]) {
    this.root = root;
  }

  get raw(): RootRaw {
    return this.root.map((t) => t.raw);
  }

  encode(): Uint8Array {
    return encode(this.raw);
  }

  toHtml(classes?: boolean): string {
    return this.root.map((t) => t.toHtml(classes)).join("");
  }
}
