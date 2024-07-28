/* eslint-disable @typescript-eslint/no-unused-vars */
import { Root, Tag } from "./daletl/types";

export function encodeTag(tag: Tag): Uint8Array {
  throw new Error("encodeTag is not implemented");
}

export function encode(root: Root): Uint8Array {
  throw new Error("encode is not implemented");
}

export function decodeTag(data: Uint8Array): Tag {
  throw new Error("decodeTag is not implemented");
}

export function decode(data: Uint8Array): Root {
  throw new Error("decode is not implemented");
}
