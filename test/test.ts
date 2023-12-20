import { assertEquals } from 'https://deno.land/std@0.209.0/assert/mod.ts';

import { levenshtein } from '../build/search_index.js';

Deno.test('test', () => {
  assertEquals(levenshtein('a', 'a'), 0);
  assertEquals(levenshtein('a', 'b'), 1);
  assertEquals(levenshtein('a', 'abc'), 2);
});
