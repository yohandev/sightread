const w = window.innerWidth;
const h = window.innerHeight;

const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
const lin = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');

svg.setAttribute('width', w);
svg.setAttribute('height', h);

lin.style.stroke = '#000';
lin.style.strokeWidth = '5px';
lin.style.fill = 'none';

svg.appendChild(lin);
document.body.appendChild(svg);

export default
{
    pts: "",
    begin()
    {
        this.pts = "";
    },
    add(frq, amp, len)
    {
        this.pts += `${frq * (w / len)}, ${h - 30 - amp * (h * 0.9)} `
    },
    end()
    {
        lin.setAttribute('points', this.pts);
    }
}