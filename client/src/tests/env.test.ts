import { expect, test } from "vitest";

test("check env", () => {
  expect(import.meta.env.MODE).toEqual("test");
});
