
import * as search from "./build/search_index.js";

console.time('search');

const charactersIndex = await Deno.readFile('./characters_index.bin');
const results = search.search_characters('luke', charactersIndex)
.map((t) => ({
  name: t.character.name,
  mediaTitle: t.character.mediaTitle,
  popularity: t.character.popularity,
  score: t.score
}));
console.log(results.slice(0, 3));


// console.time('start');
// const mediaIndex = await Deno.readFile('./media_index.bin');
// const results = search.search_media('Konosuba', mediaIndex)
// .map((t) => ({
//   name: t.media.title,
//   popularity: t.media.popularity,
//   score: t.score
// }));
// console.log(results.slice(0, 3));


console.timeEnd('search');


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
