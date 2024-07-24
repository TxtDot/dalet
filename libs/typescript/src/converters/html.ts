import { Root, Tag } from "../daletl/types";

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

function comp(
  name: string,
  body: string | Tag[],
  classNames: string,
  classes: boolean,
  props: Props = {}
): string {
  let realBody = Array.isArray(body)
    ? body.map((b) => htmlTag(b, classes)).join("")
    : body;
  return hy(name, realBody, {
    class: classes ? classNames : undefined,
    ...props,
  });
}

function element(tag: Tag, classes: boolean): string {
  return comp("section", tag.body, "el", classes);
}

function heading(tag: Tag, classes: boolean): string {
  if (typeof tag.argument !== "number") {
    return comp("h1", tag.body, "h", classes);
  }

  return comp(`h${tag.argument}`, tag.body, `h hl${tag.argument}`, classes);
}

function htmlTag(tag: Tag, classes: boolean): string {
  return [element, heading][tag.id](tag, classes);
}

export function toHtml(root: Root, classes: boolean = true): string {
  return root.map((t) => htmlTag(t, classes)).join("");
}
