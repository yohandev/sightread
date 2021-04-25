use rustfft::{ FftPlanner, num_complex::Complex };

/// apply the fast fourier transform algorithm to the buffer
/// of samples at the given `ptr`, in wasm memory, of given
/// length `len` and return the most dominant value by norm
#[no_mangle]
pub fn fft_max(ptr: i32, len: i32) -> f32
{
    // reconstruct the PCM
    let buf: &[f32] = unsafe { crate::mem::reconstruct_slice(ptr, len) };
    // get the appropriate FFT algorithm
    let fft = FftPlanner::<f32>::new().plan_fft_forward(len as usize);
    // convert to complex number
    let mut pcm = buf
        .iter()
        .map(|sample| Complex::new(*sample, 0.0))
        .collect::<Vec<_>>();
    
    // run transform
    fft.process(&mut pcm);

    // most dominant frequency
    pcm[0..len as usize / 2]
        .iter()
        .max_by(|a, b| a
            .norm_sqr()
            .partial_cmp(&b.norm())
            .unwrap_or(std::cmp::Ordering::Equal)
        )
        .map(|max| max.norm())
        .unwrap_or_default()
}

/// performs the FFT to the `f32[]` at `ptr` and of
/// length `len`, leaving the frequencies in-place and
/// amplitudes at `norm_ptr` which should also be of length
/// `len`
#[no_mangle]
pub fn fft(ptr: i32, norm_ptr: i32, len: i32)
{
    // reconstruct the PCM
    let buf: &mut [f32] = unsafe { crate::mem::reconstruct_slice_mut(ptr, len) };
    // ...and the norm buffer
    let nor: &mut [f32] = unsafe { crate::mem::reconstruct_slice_mut(norm_ptr, len) };

    // get the appropriate FFT algorithm
    let fft = FftPlanner::<f32>::new().plan_fft_forward(len as usize);
    // convert to complex number
    let mut pcm = buf
        .iter()
        .map(|sample| Complex::new(*sample, 0.0))
        .collect::<Vec<_>>();
    
    // run transform
    fft.process(&mut pcm);

    // modify in-place
    for ((fq, amp), smp) in buf.iter_mut().zip(nor.iter_mut()).zip(pcm.iter())
    {
        *fq = smp.re;
        *amp = smp.norm();
    }
}

#[no_mangle]
pub fn in_place(re_ptr: i32, im_ptr: i32, len: i32)
{
    // reconstruct the PCM
    let re: &mut [f32] = unsafe { crate::mem::reconstruct_slice_mut(re_ptr, len) };
    let im: &mut [f32] = unsafe { crate::mem::reconstruct_slice_mut(im_ptr, len) };

    // get the appropriate FFT algorithm
    let fft = FftPlanner::<f32>::new().plan_fft_forward(len as usize);
    // convert to complex number
    let mut pcm = re
        .iter()
        .map(|sample| Complex::new(*sample, 0.0))
        .collect::<Vec<_>>();
    
    // run transform
    fft.process(&mut pcm);

    // write back
    for ((re, im), complex) in re.iter_mut().zip(im.iter_mut()).zip(pcm.iter())
    {
        *re = complex.re;
        *im = complex.im;
    }
}