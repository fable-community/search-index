// import { assertEquals } from "https://deno.land/std@0.220.1/assert/assert_equals.ts";
import { assertSnapshot } from "https://deno.land/std@0.220.1/testing/snapshot.ts";

import * as search from "./build/search_index.js";


Deno.test('search characters with query "Luke"', async (test) => {
  const charactersIndex = await Deno.readFile('./characters_index.bin');
  const results = search.search_characters('luke', charactersIndex)
  .map((t) => ({
    name: t.character.name,
    mediaTitle: t.character.mediaTitle,
    popularity: t.character.popularity,
    score: t.score
  }));
  await assertSnapshot(test,results);
});


Deno.test('search characters with query "Aqua"', async (test) => {
  const charactersIndex = await Deno.readFile('./characters_index.bin');
  const results = search.search_characters('Aqua', charactersIndex)
  .map((t) => ({
    name: t.character.name,
    mediaTitle: t.character.mediaTitle,
    popularity: t.character.popularity,
    score: t.score
  }));
  await assertSnapshot(test,results);
});


Deno.test('search characters with query "Megumi"', async (test) => {
  const charactersIndex = await Deno.readFile('./characters_index.bin');
  const results = search.search_characters('Megumi', charactersIndex)
  .map((t) => ({
    name: t.character.name,
    mediaTitle: t.character.mediaTitle,
    popularity: t.character.popularity,
    score: t.score
  }));
  await assertSnapshot(test,results);
});


Deno.test('search media with query "Konosuba"', async (test) => {
  const mediaIndex = await Deno.readFile('./media_index.bin');
  const results = search.search_media('Konosuba', mediaIndex)
  .map((t) => ({
    name: t.media.title,
    popularity: t.media.popularity,
    score: t.score
  }));
  await assertSnapshot(test,results);
});




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
