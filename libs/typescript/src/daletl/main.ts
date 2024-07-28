import { encode, decode } from "../daletpack";
import { ParseError, CommonTag, Body, Root, Tag, CommonBody } from "./types";
import { TagNormalizers } from "./normalizers";

export function parseTag(tag: Tag): CommonTag {
  return TagNormalizers[tag.id](tag);
}

export function parseBody(body: Body): CommonBody {
  if (body === null) {
    return null;
  }

  if (typeof body === "string") {
    return body;
  }

  return body.map((t) => parseTag(t));
}

export function parse(root_data: Uint8Array): RootClass {
  const root = decode(root_data);

  if (!Array.isArray(root)) {
    throw new ParseError("Daletl root must be array");
  }

  return new RootClass(root.map(parseTag));
}

export class RootClass {
  root: CommonTag[];
  constructor(root: CommonTag[]) {
    this.root = root;
  }

  get raw(): Root {
    return this.root.map((t) => t.raw);
  }

  encode(): Uint8Array {
    return encode(this.raw);
  }

  toHtml(classes?: boolean): string {
    return this.root.map((t) => t.toHtml(classes)).join("");
  }
}
