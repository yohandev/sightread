const w = window.innerWidth;
const h = window.innerHeight;

const c = document.body.appendChild(document.createElement('canvas'));

c.width = w;
c.height = 1024 / 2;

const d = c.getContext('2d');

d.strokeStyle = '#ff0000';
d.lineWidth = 1;

let x = -1, y = -1;
/** @type {ImageData} */
let col = undefined;

export default
{
    step()
    {
        if (col)
        {
            d.putImageData(col, x, 0);
        }

        y = -1;
        x = (x + 1) % w;

        col = d.getImageData(x, 0, 1, c.height);

        d.beginPath();
        d.moveTo((x + 1) % w, 0);
        d.lineTo((x + 1) % w, c.height);
        d.stroke();
    },
    put(val)
    {
        y += 1;

        val = Math.round(Math.max(Math.min(val, 255 * 3), 0));

        col.data[y * 4 + 0] = val;
        col.data[y * 4 + 1] = Math.max(val -= 255, 0);
        col.data[y * 4 + 2] = Math.max(val -= 255, 0);
        col.data[y * 4 + 3] = 255;
    },

}