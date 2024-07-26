import { bodyToRaw, chtml } from "../../utils";
import { RawTag, Tag } from "../types";

export default class El extends Tag {
  constructor(body: string | Tag[]) {
    super(0, body, null);
  }

  get raw(): RawTag {
    return bodyToRaw(this.body)!;
  }

  toHtml(classes?: boolean): string {
    return chtml("section", "el", classes, this.body);
  }
}
