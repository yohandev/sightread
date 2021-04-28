import load from './mod.wasm'

/** @type {WebAssembly.Exports} */
let exports = undefined;

/** bindings over WASM module */
export default
{
    /** begins loading the WASM module */
    async load()
    {
        exports = await load();
    },

    /** @returns {WebAssembly.Memory} */
    get memory() { return exports.memory },

    /**
     * add two numbers together
     * @param {number} a 
     * @param {number} b 
     * @returns {number}
     */
    add(a, b)
    {
        return exports.add(a, b)
    }
}