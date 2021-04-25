mod mem;

/// tests for how much of a frequency is in an audio sample
/// at addres `ptr` and of length `len`, returning a confidence
/// level
/// 
/// runs `n` iteration attempts to find the phase
/// requires sample rate `fs`(for frequency sample)
#[no_mangle]
pub fn freq_amt(ptr: i32, len: i32, freq: f32, n: i32, fs: i32) -> f32
{
    let buf: &mut [f32] = unsafe { mem::reconstruct_slice_mut(ptr, len) };
    let fs = fs as f32;

    // go from phase 0 to <wavelength>:
    //  zip generated wave iterator with samples:
    //    score <- riemann sum of their products
    //  stop if score is less than previous score
    // return score
    let mut wave = Wave { freq, fs, ..Default::default() };

    // TODO optmization:
    //  check 3 tries window for a local minima or maxima,
    //  both of which the best phase can be derived from

    let mut score = 0.0f32;
    let mut phase = 0.0;
    for _ in 0..n
    {
        // the chad div trait vs virgin '/' syntax
        use std::ops::Div;

        // reset wave
        wave.i = 0.0;

        // riemann sum
        let area: f32 = buf
            .iter()
            .zip(wave)
            .map(|(a, b)| a * b)
            .sum::<f32>()
            .div(fs);
        // take the best score
        if area > score
        {
            score = area;
            phase = wave.phase;
        }

        // try again with a different phase
        wave.phase += ((1.0 / freq) / n as f32) * fs;
    }
    // normalize by dividing by average volume
    let sum: f32 = buf.iter().map(|n| n.abs()).sum();
    score /= sum / len as f32;

    // very lazy: encode second return, the phase shift, at
    // index 0 of the input buffer
    buf[0] = phase;
    // return the score with the best phase shift it
    // could find
    score
}

/// infinite iterator for sine wave of amplitude 1
#[derive(Debug, Default, Clone, Copy)]
struct Wave
{
    /// phase shift of the wave, in wavelength units
    phase: f32,
    /// frequency of the wave, in hertz
    freq: f32,

    /// current sample
    i: f32,
    /// sample rate, ie. 44_100hz
    fs: f32,
}

impl Iterator for Wave
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item>
    {
        const PI_TIMES_2: f32 = std::f32::consts::PI * 2.0;

        self.i += 1.0;
        Some(((PI_TIMES_2 * self.freq * (self.i - self.phase)) / self.fs).sin())
    }
}