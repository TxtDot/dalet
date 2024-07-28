import { Body, CommonBody, CommonTag, Tag } from "./daletl/types";

export function bodyToRaw(body: CommonBody): Body {
  if (body === null) {
    return null;
  }

  if (typeof body === "string") {
    return body;
  }

  return body.map((t) => t.raw);
}

export function bodyToHtml(body: CommonBody): string {
  if (typeof body === "string") {
    return body;
  }

  if (Array.isArray(body)) {
    return body.map((t) => t.toHtml()).join("");
  }

  return "";
}

export function getRaw(t: CommonTag): Tag {
  return t.raw;
}

interface Props {
  [key: string]: string | undefined;
}

export function chtml(
  tag: string,
  classNames: string,
  classes: boolean = true,
  body?: CommonBody,
  props?: Props
) {
  const classProp = classes ? { class: classNames } : {};
  return html(tag, body, { ...props, ...classProp });
}

function html(tag: string, body?: CommonBody, props?: Props) {
  const pr = Object.entries(props || {})
    .map(([key, value]) => `${key}="${value}"`)
    .join(" ");

  if (!body) {
    return `<${tag}${pr ? " " + pr : ""}/>`;
  }

  return `<${tag}${pr ? " " + pr : ""}>${bodyToHtml(body)}</${tag}>`;
}
