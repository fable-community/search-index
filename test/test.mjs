import { levenshtein } from "../build/search_index.js";

test("levenshtein distance calculation", () => {
  expect(levenshtein("a", "a")).toBe(0);
  expect(levenshtein("a", "b")).toBe(1);
  expect(levenshtein("a", "abc")).toBe(2);
});
