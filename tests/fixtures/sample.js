import { EventEmitter } from "events";

export function greet(name) {
  return `Hello, ${name}!`;
}

export const MAX_SIZE = 1024;

export const config = {
  name: "default",
  value: 0,
};

export class Service {
  name;
  #secret;

  constructor(name) {
    this.name = name;
    this.#secret = "";
  }

  start() {
    console.log("starting");
  }

  #stop() {
    console.log("stopping");
  }
}

export default function main() {
  console.log("hello");
}

function helper() {
  return 42;
}

export { helper };
