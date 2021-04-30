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

    let arr = wasm.alloc(4096);

    // piano A note wave
    for (let i = 0; i < arr.len; i++)
    {
        arr.f32[i] = wave(i, 440.0, 44_100);
    }

    // FFT
    let fqAmp = wasm.frequencies(arr, 44_100);

    const w = window.innerWidth;
    const h = window.innerHeight;

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    const lin = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');

    svg.setAttribute('width', w);
    svg.setAttribute('height', h);

    lin.style.stroke = '#000';
    lin.style.strokeWidth = '5px';

    svg.appendChild(lin);
    document.body.appendChild(svg);
    
    let pts = "";
    for (let i = 0; i < arr.len; i += 2)
    {
        const frq = fqAmp.f32[i] * (w / arr.len);
        const amp = h - 30 - fqAmp.f32[i + 1] * (h * 0.9);

        pts += `${frq}, ${amp} `
    }
    lin.setAttribute('points', pts);

    wasm.dealloc(arr);
}

main();