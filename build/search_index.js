import * as wasm from "./search_index_bg.wasm";
import { __wbg_set_wasm } from "./search_index_bg.js";
__wbg_set_wasm(wasm);
export * from "./search_index_bg.js";
