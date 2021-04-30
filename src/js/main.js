import wasm from '../rs/mod'

const wave = (i, f, fs) =>
{
    // overtones for a piano key
    const OVERTONES = [1.0, 0.389045, 0.063095, 0.1, 0.050699, 0.017782, 0.0204173];
    // simple sine wave
    const sine = (fq) =>
    {
        return Math.sin((2 * Math.PI * fq * i) / fs);
    }
    // go through each overtone
    return OVERTONES
        .map((amp, i) => amp * sine(f * Math.pow(2, i)))
        .reduce((a, b) => a + b, 0);
}

const main = async () =>
{
    await wasm.load()

    let arr = wasm.alloc(1024);

    // piano A note wave
    for (let i = 0; i < arr.len; i++)
    {
        arr.f32[i] = wave(i, 440.0, 44_100);
    }

    // FFT
    let fqAmp = wasm.frequencies(arr, 44_100);

    for (let i = 0; i < arr.len; i += 2)
    {
        if (fqAmp.f32[i + 1] > 1)
        {
            console.log(`${fqAmp.f32[i]}hz -> ${fqAmp.f32[i + 1]}`);
        }
    }

    wasm.dealloc(arr);
}

main();