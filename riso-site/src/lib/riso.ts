import { browser } from '$app/env';

let wasm = null;
async function initializeWasm() {
    if (browser) {
        let module = await import("riso-wasm");
        await module.default("/wasm/riso.wasm");

        wasm = {
            iso3166: module.Riso3166,
            iso4217: module.Riso4217,
            iso639: module.Riso639,
        }
    }
}

export default async function () {
    if (wasm === null) {
        await initializeWasm();
    }

    return Promise.resolve(wasm);
}
