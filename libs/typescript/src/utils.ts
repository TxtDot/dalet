import { Body, ParseError, RawBody, RawTag, Tag } from "./daletl/types";

export function bodyToRaw(body: Body): RawBody {
  if (typeof body === "string") {
    return body;
  }

  if (Array.isArray(body)) {
    if (Array.isArray(body[0])) {
      return body.map(getRaw);
    }
  }

  return null;
}

export function getRaw(t: Tag): RawTag {
  return t.raw;
}

interface Props {
  [key: string]: string;
}

function hy(tag: string, body?: string, props?: Props) {
  return `<${tag} ${
    props
      ? Object.entries(props)
          .map(([key, value]) => `${key}="${value}"`)
          .join(" ")
      : ""
  }>${body}</${tag}>`;
}
