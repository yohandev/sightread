import * as module from '../rs/src/lib.rs'
import microphone from './mic';

/**
 * rust crate's exports
 */
const crate =
{
    /**
     * mem.rs module
     */
    alloc:
    {
        /**
         *  allocate an i32[] of length `len`
         * 
         * @param {Number} len length of array allocated
         */
        "i32[]": (len) =>
        {
            const ptr = module.alloc_arr_i32(len);

            return new crate.alloc.arr(ptr, len);
        },
        /**
         *  allocate an f32[] of length `len`
         * 
         * @param {Number} len length of array allocated
         */
        "f32[]": (len) =>
        {
            const ptr = module.alloc_arr_f32(len);
 
            return new crate.alloc.arr(ptr, len);
        },
        /**
         * wrapper around a reference to an array in wasm memory
         */
        arr: class WasmArray
        {
            /**
             * construct a new array reference
             * @param {Number} ptr pointer in bytes to start
             * @param {Number} len length, in elements, of slice
             */
            constructor(ptr, len)
            {
                this.ptr = ptr;
                this.len = len;
            }

            /**
             * get the underlying buffer, interpreted as i32[]
             */
            get i32() { return new Int32Array(module.memory.buffer, this.ptr, this.len); }

            /**
             * get the underlying buffer, interpreted as f32[]
             */
            get f32() { return new Float32Array(module.memory.buffer, this.ptr, this.len); }
        }
    },
    /**
     * calculate a relative score for a given frequency in an audio buffer
     * 
     * @param {WasmArray} buf audio samples buffer
     * @param {Number} freq frequency to test for
     * @param {Number} iter number of test iterations
     */
    freqAmount: (buf, freq, iter) =>
    {
        return module.freq_amt(buf.ptr, buf.len, freq, iter, microphone.sampleRate());
    }
}
export default crate;