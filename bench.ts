import * as search from './build/search_index.js';

console.time('elapsed');

// const mediaIndex = await Deno.readFile('./media_index.bin');
const charactersIndex = await Deno.readFile('./characters_index.bin');

// const results1 = search.search_characters('Sayuri Haruno', charactersIndex)
//   .map((t) => ({
//     name: t.name,
//     mediaTitle: t.mediaTitle,
//     popularity: t.popularity,
//   }));
// console.log(results1.slice(0, 1));

// const results2 = search.search_characters('Aqua', charactersIndex)
//   .map((t) => ({
//     name: t.name,
//     mediaTitle: t.mediaTitle,
//     popularity: t.popularity,
//   }));
// console.log(results2.slice(0, 1));

const results3 = search.search_characters('Jahy', charactersIndex)
  .map((t) => ({
    name: t.name,
    mediaTitle: t.mediaTitle,
    popularity: t.popularity,
  }));
console.log(results3.slice(0, 1));

// const results4 = search.search_characters('gura', charactersIndex, [
//   new Character(
//     'hololive:gura',
//     'hololive:myth',
//     ['Gawr Gura'],
//     ['Hololive'],
//     1_000_000,
//     5,
//     'MAIN',
//   ),
// ])
//   .map((t) => ({
//     name: t.name,
//     popularity: t.popularity,
//   }));
// console.log(results4.slice(0, 2));

// const results5 = search.search_media('Konosuba', mediaIndex)
//   .map((t) => ({
//     name: t.title,
//     popularity: t.popularity,
//   }));
// console.log(results5.slice(0, 3));

// const results6 = search.id_mapped_filter_characters(
//   charactersIndex,
//   undefined,
//   undefined,
//   1000,
//   50_000,
//   undefined,
// );

// console.log(Array.from(results6.entries()).slice(0, 3));

console.timeEnd('elapsed');
