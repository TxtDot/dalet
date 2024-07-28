import { bodyToRaw, chtml } from "../../utils";
import { CommonTag, Tag } from "../types";

export default class El extends CommonTag {
  constructor(body: CommonTag[]) {
    super(0, body, null);
  }

  get raw(): Tag {
    return { id: this.id, body: bodyToRaw(this.body), argument: null };
  }

  toHtml(classes?: boolean): string {
    return chtml("section", "el", classes, this.body);
  }
}
