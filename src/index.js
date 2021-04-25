import { SVG } from '@svgdotjs/svg.js';

import microphone from './mic';
import crate from './crate';

const buf = crate.alloc['f32[]'](2048);

const svg = SVG()
    .addTo('body')
    .width(window.innerWidth)
    .height(window.innerHeight);
const ln1 = svg
    .polyline()
    .fill('none')
    .stroke({ width: 3, color: 'black' });
const ln2 = svg
    .polyline()
    .fill('none')
    .stroke({ width: 3, color: 'red' });

const wave = (i, freq, phase) =>
{
    //((PI_TIMES_2 * self.freq * (self.i - self.phase)) / self.fs).sin()
    return Math.sin((2 * Math.PI * freq * (i - phase)) / microphone.sampleRate());
}

microphone.listen(buf.len, x =>
{
    // upload to wasm
    buf.f32.set(x);

    let { score, phase } = crate.freqAmount(buf, 261, 10);
    
    console.log(`440hz: ${score * 100}`);

    let pts1 = [];
    let pts2 = [];
    for (let i = 0; i < x.length; i++)
    {
        // x
        pts1.push(i / x.length * window.innerWidth);
        // y
        pts1.push(x[i] * window.innerHeight * 5);

        // x
        pts2.push(i / x.length * window.innerWidth);
        // y
        pts2.push(wave(i, 261, phase) * window.innerHeight * 0.2);
    }
    ln1.plot(pts1);
    ln2.plot(pts2);
})
