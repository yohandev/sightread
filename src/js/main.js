import wasm from '../rs/mod'
import draw from './draw';
import mic from './mic';

const main = async () =>
{
    await wasm.load();

    const pcm = wasm.alloc(1024);
    const old = new Float32Array(pcm.len / 2);

    await mic(pcm.len, (samples, fs) =>
    {
        // for old = [staging buffer], pcm = [fft buffer]
        //
        // 1. copy [staging buffer] to left of [fft buffer]
        // 2. copy left of [new pcm] to right of [fft buffer]
        // 3. copy right of [new pcm] to [staging buffer]
        // 4. apply windowing fn to [fft buffer]
        // 5. run fft on [fft buffer]

        // left, right
        const lfft = pcm.f32.subarray(0, pcm.len / 2);
        const rfft = pcm.f32.subarray(pcm.len / 2);
        const lnew = samples.subarray(0, samples.length / 2);
        const rnew = samples.subarray(samples.length / 2);

        // 1.
        lfft.set(old);
        // 2.
        rfft.set(lnew);
        // 3.
        old.set(rnew);
        // 4.
        wasm.hamming(pcm);
        // 5.
        wasm.frequencies(pcm, fs);
        // 6.
        //wasm.decibel(pcm, 30);

        // draw
        draw.step();
        for (let i = 0; i < pcm.len; i += 2)
        {
            const val = Math.log10(pcm.f32[i + 1] * 1000) * 500; // no decibel
            //const  val = (50 + pcm.f32[i + 1]) * 10; // decibel
            draw.put(val);
        }
    })
    //wasm.dealloc(arr);
}

main();