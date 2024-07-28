import { chtml } from "../../utils";
import { CommonTag } from "../types";

export default class Heading extends CommonTag {
  constructor(body: CommonTag[], argument?: number | null) {
    super(1, body, argument || null);
  }

  toHtml(classes?: boolean): string {
    return chtml(
      `h${this.argument || 1}`,
      `h hl${this.argument || 1}`,
      classes,
      this.body
    );
  }
}
