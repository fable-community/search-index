import { assertEquals } from 'https://deno.land/std@0.209.0/assert/mod.ts';

import { search } from '../build/search_index.js';

Deno.test('test', () => {
  assertEquals(search('a', 'b'), 0.0);
});
