import { encode, decode } from "@msgpack/msgpack";
import { ParseError, Root, Tag } from "./types";

export function parseRawTag(raw_tag: any[] | string): Tag {
  if (typeof raw_tag === "string") {
    return {
      id: 0,
      body: raw_tag,
      argument: null,
    };
  }

  if (Array.isArray(raw_tag)) {
    if (Array.isArray(raw_tag[0])) {
      return {
        id: 0,
        body: parseRawTagBody(raw_tag),
        argument: null,
      };
    }

    if (typeof raw_tag[0] === "number") {
      return {
        id: raw_tag[0],
        body: parseRawTagBody(raw_tag[1]),
        argument: raw_tag[3] || null,
      };
    }
  }

  throw new ParseError("Invalid tag");
}

export function parseRawTagBody(body: string | null | any[]): Tag["body"] {
  if (typeof body === "string") {
    return body;
  }

  if (body === null) {
    return null;
  }

  if (Array.isArray(body)) {
    if (Array.isArray(body[0])) {
      return body.map(parseRawTag);
    }

    if (typeof body[0] === "number") {
      return [parseRawTag(body)];
    }
  }

  throw new ParseError("Invalid tag body");
}

export function parse(root_data?: Uint8Array): Root {
  let root = root_data ? decode(root_data) : [];

  if (!Array.isArray(root)) {
    throw new ParseError("Daletl root must be array");
  }

  return root.map(parseRawTag);
}
