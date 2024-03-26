import { test } from './build/search_index.js';

console.time('searching index');
await test('luka');
console.timeEnd('searching index');

// deno-lint-ignore prefer-ascii
// test('げんまい茶');
