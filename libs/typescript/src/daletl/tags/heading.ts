import { chtml } from "../../utils";
import { Tag } from "../types";

export default class Heading extends Tag {
  constructor(body: string, argument?: number | null) {
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
