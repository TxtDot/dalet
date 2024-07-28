import { parseBody } from "./main";
import El from "./tags/el";
import Heading from "./tags/heading";
import { z } from "zod";
import { Tag } from "./types";

const textOrTag = z.custom((b) => b !== null);
const text = z.string();

const TagNormalizers = [
  n(textOrTag, z.any(), El),
  n(text, z.number().int().min(1).max(6).nullable(), Heading),
];

export { TagNormalizers };

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function n(body: z.ZodTypeAny, argument: z.ZodTypeAny, T: any) {
  return (tag: Tag) => {
    const parsedBody = parseBody(tag.body);

    z.object({ id: z.number().int(), body, argument }).parse({
      id: tag.id,
      body: parsedBody,
      argument,
    });

    return new T(parsedBody, argument);
  };
}
