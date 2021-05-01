import wasm from '../rs/mod'
import draw from './draw';
import mic from './mic';

const main = async () =>
{
    await wasm.load();

    const pcm = wasm.alloc(1024);

    await mic(1024, (samples, fs) =>
    {
        // copy over data
        pcm.f32.set(samples);
        // FFT
        let fft = wasm.frequencies(wasm.hamming(pcm), fs);
        
        // draw
        draw.step();
        for (let i = 0; i < pcm.len; i += 2)
        {
            draw.put(fft.f32[i + 1] * 10000);
        }
    })
    //wasm.dealloc(arr);
}

main();