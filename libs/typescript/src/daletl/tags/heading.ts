import { chtml } from "../../utils";
import { Tag } from "../types";

export default class Heading extends Tag {
  constructor(body: string, argument: number) {
    super(1, body, argument);
  }

  toHtml(classes?: boolean): string {
    return chtml(
      `h${this.argument}`,
      `h hl${this.argument}`,
      classes,
      this.body
    );
  }
}
