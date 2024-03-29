import * as search from './build/search_index.js';

console.time('elapsed');

const charactersIndex = await Deno.readFile('./characters_index.bin');
const results = search.search_characters('megumin', charactersIndex)
  .map((t) => ({
    name: t.name,
    mediaTitle: t.mediaTitle,
    popularity: t.popularity,
  }));
console.log(results.slice(0, 3));

// console.time('start');
// const mediaIndex = await Deno.readFile('./media_index.bin');
// const results = search.search_media('Konosuba', mediaIndex)
// .map((t) => ({
//   name: t.title,
//   popularity: t.popularity,
// }));
// console.log(results.slice(0, 3));

// const charactersIndex = await Deno.readFile('./characters_index.bin');
// const results = search.filter_characters(
//   undefined,
//   500_000,
//   510_000,
//   5,
//   charactersIndex,
// )
//   .map((t) => ({
//     name: t.name,
//     mediaTitle: t.mediaTitle,
//     popularity: t.popularity,
//   }));
// console.log(results.slice(0, 3));

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
