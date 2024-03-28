/* tslint:disable */
/* eslint-disable */
/**
* @param {string} json
* @returns {Uint8Array}
*/
export function create_characters_index(json: string): Uint8Array;
/**
* @param {string} query
* @param {Uint8Array} index_file
* @returns {(Character)[]}
*/
export function search_characters(query: string, index_file: Uint8Array): (Character)[];
/**
* @param {string} json
* @returns {Uint8Array}
*/
export function create_media_index(json: string): Uint8Array;
/**
* @param {string} query
* @param {Uint8Array} index_file
* @returns {(Media)[]}
*/
export function search_media(query: string, index_file: Uint8Array): (Media)[];
/**
*/
export enum CharacterRole {
  MAIN = 0,
  SUPPORTING = 1,
  BACKGROUND = 2,
}
/**
*/
export class Character {
  free(): void;
/**
*/
  id: string;
/**
*/
  mediaTitle: (string)[];
/**
*/
  name: (string)[];
/**
*/
  popularity: number;
/**
*/
  rating: number;
/**
*/
  role: CharacterRole;
}
/**
*/
export class Media {
  free(): void;
/**
*/
  id: string;
/**
*/
  popularity: number;
/**
*/
  title: (string)[];
}
/**
*/
export class MediaResult {
  free(): void;
/**
*/
  media: Media;
/**
*/
  score: number;
}
