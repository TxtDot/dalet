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

export function bodyToHtml(body: Body): string {
  if (typeof body === "string") {
    return body;
  }

  if (Array.isArray(body)) {
    return body.map((t) => t.toHtml()).join("");
  }

  return "";
}

export function getRaw(t: Tag): RawTag {
  return t.raw;
}

interface Props {
  [key: string]: string | undefined;
}

export function chtml(
  tag: string,
  classNames: string,
  classes: boolean = true,
  body?: Body,
  props?: Props
) {
  let classProp = classes ? { class: classNames } : {};
  return html(tag, body, { ...props, ...classProp });
}

function html(tag: string, body?: Body, props?: Props) {
  let pr = Object.entries(props || {})
    .map(([key, value]) => `${key}="${value}"`)
    .join(" ");

  if (!body) {
    return `<${tag}${pr ? " " + pr : ""}/>`;
  }

  return `<${tag}${pr ? " " + pr : ""}>${bodyToHtml(body)}</${tag}>`;
}
