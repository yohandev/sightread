import * as module from '../rs/src/lib.rs'

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

            return new Int32Array(module.memory.buffer, ptr, len);
        },
        /**
         *  allocate an f32[] of length `len`
         * 
         * @param {Number} len length of array allocated
         * @returns {f32Arr} allocated array reference
         */
        "f32[]": (len) =>
        {
            const ptr = module.alloc_arr_f32(len);
 
            return new f32Arr(ptr, len);
        }
    },
    fft:
    {
        /**
         * applies the fourier transform to the given buffer of audio samples,
         * and returns the maximum value by norm
         * 
         * @param {f32Arr} buf f32[] in wasm memory
         * @returns {Number} frequency with the greatest amplitude
         */
        fftMax: (buf) =>
        {
            return module.fft_max(buf.ptr, buf.len);
        },
        /**
         * applies the fourier transform to the given buffer of audio samples,
         * writing frequencies in-place and amplitudes at `amp` buffer
         * @param {f32Arr} buf 
         * @param {f32Arr} amp 
         */
        freq: (buf, amp) =>
        {
            console.assert(buf.len == amp.len);

            module.fft(buf.ptr, amp.ptr, buf.len);
        },
        fft: (re, im) =>
        {
            module.in_place(re.ptr, im.ptr, re.len);
        }
    },
}

class f32Arr
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
     * get the underlying buffer, guaranteed to be valid
     */
    get buf()
    {
        return new Float32Array(module.memory.buffer, this.ptr, this.len);
    }
}

export default crate;