use microfft::real::*;

/// extracts the frequencies out of a signal 
///
/// - in: PCM buffer
///     - `f32[]` at `ptr..ptr + len * sizeof(f32)`
///     - `fₛ`, sampling frequency
/// - out: (Frequency, Amplitude) buffer
///     - `(f32, f32)[]` at `ptr..ptr + len * sizeof(f32)`
///     - overwrites input buffer
///     - frequency in hertz, amplitude normalized
///
/// only works on sizes, `len`, of 4, 8, 16, 32, 64, 128,
/// 256, 512, 1024, 2048, 4096
#[no_mangle]
pub fn frequencies(ptr: u32, len: u32, fs: u32)
{
    use std::slice::from_raw_parts_mut as raw_slice;

    /// reconstruct sized array from its pointer
    fn cast_arr<'a, const N: usize>(ptr: u32) -> &'a mut [f32; N]
    {
        unsafe { &mut *(ptr as *mut [f32; N]) }
    }

    // frequency resolution = sampling frequency / samples
    let f_res = fs as f32 / len as f32;
    // spectrum for various sizes
    let spectrum: &mut [_] = match len
    {
        4 => rfft_4(cast_arr::<4>(ptr)),
        8 => rfft_8(cast_arr::<8>(ptr)),
        16 => rfft_16(cast_arr::<16>(ptr)),
        32 => rfft_32(cast_arr::<32>(ptr)),
        64 => rfft_64(cast_arr::<64>(ptr)),
        128 => rfft_128(cast_arr::<128>(ptr)),
        256 => rfft_256(cast_arr::<256>(ptr)),
        512 => rfft_512(cast_arr::<512>(ptr)),
        1024 => rfft_1024(cast_arr::<1024>(ptr)),
        2048 => rfft_2048(cast_arr::<2048>(ptr)),
        4096 => rfft_4096(cast_arr::<4096>(ptr)),
        _ => unreachable!()
    };
    // nyquist frequency
    spectrum[0].im = 0.0;

    // NOTE:
    // `out` aliases `spectrum` by virtue of how microfft
    // works
    let out = unsafe { raw_slice(ptr as *mut (f32, f32), len as usize / 2) };

    // write back the frequencies and their amplitude
    for (i, ((fq, amp), c)) in out.iter_mut().zip(spectrum.iter()).enumerate()
    {
        // equivalent to:
        // c.im = c.norm_sqr() # aliasing
        *amp = 2.0 * c.norm_sqr().sqrt() / len as f32;
        // at this point, neither c or fq_amp is usable
        *fq = f_res * i as f32;
    }
}