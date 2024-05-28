/* tslint:disable */
/* eslint-disable */
/**
* @param {string} json
* @returns {Uint8Array}
*/
export function create_characters_index(json: string): Uint8Array;
/**
* @param {string} query
* @param {Uint8Array | undefined} [index_file]
* @param {(Character)[] | undefined} [extra]
* @returns {(Character)[]}
*/
export function search_characters(query: string, index_file?: Uint8Array, extra?: (Character)[]): (Character)[];
/**
* @param {Uint8Array | undefined} [index_file]
* @param {(Character)[] | undefined} [extra]
* @param {string | undefined} [role]
* @param {number | undefined} [popularity_lesser]
* @param {number | undefined} [popularity_greater]
* @param {number | undefined} [rating]
* @returns {Map<any, any>}
*/
export function media_mapped_filter_characters(index_file?: Uint8Array, extra?: (Character)[], role?: string, popularity_lesser?: number, popularity_greater?: number, rating?: number): Map<any, any>;
/**
* @param {Uint8Array | undefined} [index_file]
* @param {(Character)[] | undefined} [extra]
* @param {string | undefined} [role]
* @param {number | undefined} [popularity_lesser]
* @param {number | undefined} [popularity_greater]
* @param {number | undefined} [rating]
* @returns {Map<any, any>}
*/
export function id_mapped_filter_characters(index_file?: Uint8Array, extra?: (Character)[], role?: string, popularity_lesser?: number, popularity_greater?: number, rating?: number): Map<any, any>;
/**
* @param {string} json
* @returns {Uint8Array}
*/
export function create_media_index(json: string): Uint8Array;
/**
* @param {string} query
* @param {Uint8Array | undefined} [index_file]
* @param {(Media)[] | undefined} [extra]
* @returns {(Media)[]}
*/
export function search_media(query: string, index_file?: Uint8Array, extra?: (Media)[]): (Media)[];
/**
*/
export class Character {
  free(): void;
/**
* @param {string} id
* @param {string | undefined} media_id
* @param {(string)[]} name
* @param {(string)[]} media_title
* @param {number} popularity
* @param {number} rating
* @param {string | undefined} [role]
*/
  constructor(id: string, media_id: string | undefined, name: (string)[], media_title: (string)[], popularity: number, rating: number, role?: string);
/**
*/
  id: string;
/**
*/
  mediaId?: string;
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
  role?: string;
}
/**
*/
export class Media {
  free(): void;
/**
* @param {string} id
* @param {(string)[]} title
* @param {number} popularity
*/
  constructor(id: string, title: (string)[], popularity: number);
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
