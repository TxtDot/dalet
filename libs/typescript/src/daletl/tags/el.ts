import { bodyToRaw } from "../../utils";
import { RawTag, Tag, Body } from "../types";

export default class El extends Tag {
  constructor(body: string | Tag[]) {
    super(0, body, null);
  }

  public get raw(): RawTag {
    return bodyToRaw(this.body);
  }

  public toHtml(classes?: boolean): string {
    return "";
  }
}
