import { parseBody } from "./main";
import El from "./tags/el";
import Heading from "./tags/heading";
import { RawTagAsArray } from "./types";
import { z } from "zod";

const TagNormalizers = [
  n(
    z.custom((b) => b !== null),
    z.any(),
    El
  ),
  n(z.string(), z.number().int().min(1).max(6), Heading),
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
