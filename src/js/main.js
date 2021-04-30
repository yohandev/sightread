import wasm from '../rs/mod'
import draw from './draw';
import mic from './mic';

const main = async () =>
{
    await wasm.load();

    const pcm = wasm.alloc(4096);

    await mic(4096, (samples, fs) =>
    {
        // copy over data
        pcm.f32.set(samples);
        // FFT
        let fft = wasm.frequencies(pcm, fs);

        // draw
        draw.begin()
        for (let i = 0; i < pcm.len; i += 2)
        {
            draw.add(fft.f32[i], fft.f32[i + 1] * 10, pcm.len);
        }
        draw.end()
    })
    //wasm.dealloc(arr);
}

main();