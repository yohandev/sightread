import crate from './crate';
import visualizeFft from './graph';
import microphone from './mic';

// /**
//  * PCM buffer in wasm memory
//  */
// const pcm = crate.alloc['f32[]'](1024);
// /**
//  * amplitudes buffer in wasm memory
//  */
// const amp = crate.alloc['f32[]'](pcm.len);

// microphone(pcm.len, (smp, hz) =>
// {
//     pcm.buf.set(smp);

//     crate.fft.freq(pcm, amp)// * (hz / buf.length);

//     graph(pcm.buf, amp.buf);
// })

const re = crate.alloc['f32[]'](2048);
const im = crate.alloc['f32[]'](2048);

microphone(re.len, (win, hz) =>
{
    re.buf.set(win);
    crate.fft.fft(re, im);

    visualizeFft(re.buf, im.buf);
})