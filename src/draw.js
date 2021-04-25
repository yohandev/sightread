import { SVG } from '@svgdotjs/svg.js';

import microphone from './mic';

// SVG canvas
const svg = SVG()
    .addTo('body')
    .width(window.innerWidth)
    .height(window.innerHeight);
// recorded line
const ln1 = svg
    .polyline()
    .fill('none')
    .stroke({ width: 3, color: 'black' });
// target line
const ln2 = svg
    .polyline()
    .fill('none')
    .stroke({ width: 3, color: 'red' });

// generate a piano sounding wave
const wave = (i, freq, phase) =>
{
    // overtones for a piano key
    const OVERTONES = [1.0, 0.389045, 0.063095, 0.1, 0.050699, 0.017782, 0.0204173];
    // simple sine wave
    const sine = fq =>
    {
        return Math.sin((2 * Math.PI * fq * (i - phase)) / microphone.sampleRate());
    }
    // go through each overtone
    return OVERTONES
        .map((amp, i) => amp * sine(freq * Math.pow(2, i)))
        .reduce((a, b) => a + b, 0);
}

/**
 * draws waves ~~~~~~
 * 
 * @param {Float32Array} pcm microphone input buffer
 * @param {Number} fq fundamental frequency demanded
 * @param {Number} phase phase of wave
 */
const draw = (pcm, fq, phase) =>
{
    let pts1 = [];
    let pts2 = [];
    for (let i = 0; i < pcm.length; i++)
    {
        // x
        pts1.push(i / pcm.length * window.innerWidth);
        // y
        pts1.push(pcm[i] * window.innerHeight * 5);

        // x
        pts2.push(i / pcm.length * window.innerWidth);
        // y
        pts2.push(wave(i, fq, phase) * window.innerHeight * 0.2);
    }
    ln1.plot(pts1);
    ln2.plot(pts2);
}
export default draw;