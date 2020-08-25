import * as wasm from './wasm_test_bg.wasm';

/**
* @param {number} n1
* @param {number} n2
* @returns {number}
*/
export function add(n1, n2) {
    var ret = wasm.add(n1, n2);
    return ret;
}

