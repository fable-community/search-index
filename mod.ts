import {
  create_characters_index,
  create_media_index,
  search_characters,
} from './build/search_index.js';

// //
// const mediaCache = await Deno.readTextFile('./media_cache.json');

// const mediaIndex = create_media_index(mediaCache);

// await Deno.writeFile('./media_index.bin', mediaIndex);
// //

// //
// const charactersCache = await Deno.readTextFile('./characters_cache.json');

// const charactersIndex = create_characters_index(charactersCache);

// await Deno.writeFile('./characters_index.bin', charactersIndex);
// //

console.time('searching index');
const charactersIndex = await Deno.readFile('./characters_index.bin');
search_characters('luka', charactersIndex);
console.timeEnd('searching index');

// deno-lint-ignore prefer-ascii
// search_characters('げんまい茶', charactersIndex);
