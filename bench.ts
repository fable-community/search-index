import * as search from './build/search_index.js';
import { Character } from './build/search_index.js';

console.time('elapsed');

const mediaIndex = await Deno.readFile('./media_index.bin');
const charactersIndex = await Deno.readFile('./characters_index.bin');

const results1 = search.search_characters('Sayuri Haruno', charactersIndex)
  .map((t) => ({
    name: t.name,
    mediaTitle: t.mediaTitle,
    popularity: t.popularity,
  }));
console.log(results1.slice(0, 1));
// console.log(results.findIndex((t) => t.name.includes('Sayuri Haruno')));

const results2 = search.search_characters('Aqua', charactersIndex)
  .map((t) => ({
    name: t.name,
    mediaTitle: t.mediaTitle,
    popularity: t.popularity,
  }));
console.log(results2.slice(0, 1));

const results = search.search_characters('gura', charactersIndex, [
  new Character(
    'hololive:gura',
    'hololive:myth',
    ['Gawr Gura'],
    ['Hololive'],
    1_000_000,
    5,
    'MAIN',
  ),
])
  .map((t) => ({
    name: t.name,
    popularity: t.popularity,
  }));
console.log(results.slice(0, 1));

// const results2 = search.search_media('Konosuba', mediaIndex)
//   .map((t) => ({
//     name: t.title,
//     popularity: t.popularity,
//   }));
// console.log(results2.slice(0, 3));

const results3 = search.filter_characters(
  charactersIndex,
  undefined,
  undefined,
  1000,
  50_000,
  undefined,
);

console.log(Array.from(results3.entries()).slice(0, 3));

console.timeEnd('elapsed');

// //
// const mediaCache = await Deno.readTextFile('./media_cache.json');

// const mediaIndex = search.create_media_index(mediaCache);

// await Deno.writeFile('./media_index.bin', mediaIndex);
// // //

// //
// const charactersCache = await Deno.readTextFile('./characters_cache.json');

// const charactersIndex = search.create_characters_index(charactersCache);

// await Deno.writeFile('./characters_index.bin', charactersIndex);
// //
