import { parseBody } from "./main";
import El from "./tags/el";
import { ParseError, RawTagAsArray } from "./types";

export function ElNormalizer(tag: RawTagAsArray): El {
  let body = parseBody(tag[1]);

  if (body == null) {
    throw new ParseError("Invalid tag body, must be not null");
  }

  return new El(body);
}

const TagNormalizers = [ElNormalizer];

export { TagNormalizers };
