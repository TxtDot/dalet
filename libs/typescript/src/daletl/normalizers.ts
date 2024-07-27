import { parseBody } from "./main";
import El from "./tags/el";
import Heading from "./tags/heading";
import { RawTagAsArray } from "./types";
import { z } from "zod";

const textOrTag = z.custom((b) => b !== null);
const text = z.string();

const TagNormalizers = [
  n(textOrTag, z.any(), El),
  n(text, z.number().int().min(1).max(6).nullable(), Heading),
];

export { TagNormalizers };

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function n(body: z.ZodTypeAny, argument: z.ZodTypeAny, T: any) {
  return (tag: RawTagAsArray) => {
    const parsedBody = parseBody(tag[1]);

    z.tuple([z.number().int(), body, argument]).parse([
      tag[0],
      parsedBody,
      tag[2],
    ]);

    return new T(parsedBody, tag[2]);
  };
}
