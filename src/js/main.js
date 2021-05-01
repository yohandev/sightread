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
        let fft = wasm.frequencies(wasm.hamming(pcm), fs);
        // frequencies past threshold
        let frq = [];

        // draw
        draw.begin()
        for (let i = 0; i < pcm.len; i += 2)
        {
            if (fft.f32[i + 1] > 0.0035)
            {
                frq.push({ frq: fft.f32[i], amp: fft.f32[i + 1] });
            }
            draw.add(fft.f32[i], Math.max(fft.f32[i + 1], 0.001) * 10, pcm.len);
        }
        // frequencies
        frq
            .sort((a, b) => b.amp - a.amp)
            .forEach(n => draw
                .text(`${n.frq.toPrecision(5)}hz - ${n.amp.toPrecision(5)}`)
            );
        // draw
        draw.end()
    })
    //wasm.dealloc(arr);
}

main();