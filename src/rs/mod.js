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

    /** a wrapper over wasm memory, with 32-bit elements */
    arr: class WasmArray
    {
        /**
         * create a new wrapper over wasm memory 
         * @param {number} ptr byte offset, aligned to 32-bits
         * @param {number} len number of elements
         */
        constructor(ptr, len)
        {
            /** @type {ArrayBuffer} */
            this.buf = () => exports.memory.buffer;

            this.ptr = ptr;
            this.len = len;
        }
        /** interpret this array as an `i32[]` */
        get i32() { return new Int32Array(this.buf(), this.ptr, this.len) }
        /** interpret this array as an `f32[]` */
        get f32() { return new Float32Array(this.buf(), this.ptr, this.len) }
    },
    /**
     * allocate a new array in WASM memory
     * @param {number} len length of array to allocate
     */
    alloc(len)
    {
        return new this.arr(exports.alloc32(len), len)
    },
    /**
     * deallocate an array allocated with `alloc` in WASM memory
     * @param {WasmArray} arr array to deallocate
     */
    dealloc(arr)
    {
        exports.dealloc32(arr.ptr, arr.len);
    },
    /**
     * extracts the frequencies out of a signal, overwriting the input
     * buffer of `f32` samples with half as many `(f32, f32)`(packed)
     * tuples representing (frequency, amplitude)
     * 
     * @param {WasmArray} pcm input samples, of some power of 2 length
     * @param {number} fs sampling frequency
     * @returns {WasmArray} (frequency, amplitude)[pcm.len / 2]
     */
    frequencies(pcm, fs)
    {
        exports.frequencies(pcm.ptr, pcm.len, fs);

        return pcm;
    }
}