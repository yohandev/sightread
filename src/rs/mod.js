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

        console.log(exports)
    },

    /** @returns {WebAssembly.Memory} */
    get memory() { return exports.memory },

    /**
     * allocate a new array in WASM memory
     * @param {'f32[]' | 'i32[]'} ty type of array to allocate
     * @param {number} len length of array to allocate
     */
    alloc(ty, len)
    {
        const ptr = exports.alloc32(len);

        switch (ty)
        {
            case 'f32[]': return new Float32Array(this.memory.buffer, ptr, len);
            case 'i32[]': return new Int32Array(this.memory.buffer, ptr, len);
        }
    },
    /**
     * deallocate an array allocated with `alloc` in WASM memory
     * @param {Float32Array | Int32Array} arr array to deallocate
     */
    dealloc(arr)
    {
        exports.dealloc32(arr.byteOffset, arr.length);
    },
}