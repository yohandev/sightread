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
         */
         "f32[]": (len) =>
         {
             const ptr = module.alloc_arr_f32(len);
 
             return new Float32Array(module.memory.buffer, ptr, len);
         }
    }
}
export default crate;