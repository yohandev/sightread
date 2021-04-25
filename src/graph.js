import { Circle, SVG } from '@svgdotjs/svg.js'

const svg = SVG()
    .addTo('body')
    .size(window.innerWidth, window.innerHeight)

/** @type {Circle[]} */
const pts = [];
for (let i = 0; i < 1024; i++)
{
    pts.push(svg
        .circle(3)
        .fill('black')
        .stroke('black')
    );
}

/**
 * 
 * @param {Float32Array} re 
 * @param {Float32Array} im 
 */
const visualizeFft = (re, im) =>
{
    const w = window.innerWidth;
    const h = window.innerHeight;

    for (let i = 0; i < pts.length; i++)
    {
        const nor = Math.sqrt((re[i] * re[i]) + (im[i] * im[i]));
        const arg = Math.atan2(re[i], im[i]);

        pts[i].x((arg * 0.05 * w) + (w * 0.5));
        pts[i].y((nor * 0.25 * h) + (h * 0.5));
    }
}
export default visualizeFft;
// const path = SVG()
//     .addTo('body')
//     .
//     .polyline()
//     .fill('none')
//     .stroke({ width: 1, color: 'black' });

// let pts = null;

// /**
//  * graph output of FFT
//  * 
//  * @param {Float32Array} fq frequencies
//  * @param {Float32Array} amp amplitudes
//  */
// const graph = (fq, amp) =>
// {
//     pts = pts || new Array(fq.length * 2);

//     const N = fq.length / 2;

//     let max = 0, pitch;
//     for (var i = 0, j = 0; i < 1 - (1 / N); i += 1 / N, j++)
//     {
//         pts[j * 2 + 0] = i * window.innerWidth;
//         pts[j * 2 + 1] = Math.pow(amp[j] * 0.01, 5) * window.innerHeight;

//         if (amp[j] >= max)
//         {
//             pitch = i;
//             max = amp[j];
//         }
//     }
//     //console.log(`${(44_000 / N) * pitch}hz`);
//     path.plot(pts);
// }
// export default graph;