import { parse, Root, El, Heading } from "./lib";

const data = new Root([
  new El("I am Element"),
  new Heading("I am heading", 1),
]).encode();

const root = parse(data);

console.log(root.raw, "\n");
console.log(root.toHtml());
console.log(root.toHtml(false));