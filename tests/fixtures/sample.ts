import { EventEmitter } from "events";

export function greet(name: string): string {
  return `Hello, ${name}!`;
}

export const MAX_SIZE = 1024;

export const config = {
  name: "default",
  value: 0,
};

export interface Reader {
  read(p: Uint8Array): number;
  close(): void;
}

export type Status = "active" | "inactive";

export type Config = {
  name: string;
  value: number;
};

export class Service {
  public name: string;
  private secret: string;

  constructor(name: string) {
    this.name = name;
    this.secret = "";
  }

  public start(): void {
    console.log("starting");
  }

  private stop(): void {
    console.log("stopping");
  }
}

export enum Color {
  Red = "red",
  Green = "green",
  Blue = "blue",
}

export default function main() {
  console.log("hello");
}

function helper() {
  return 42;
}

export { helper };
