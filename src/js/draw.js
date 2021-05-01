const w = window.innerWidth;
const h = window.innerHeight;

const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
const lin = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');
const txt = document.createElementNS('http://www.w3.org/2000/svg', 'foreignObject');
const div = document.createElement('p');

svg.setAttribute('width', w);
svg.setAttribute('height', h);

lin.style.stroke = '#000';
lin.style.strokeWidth = '5px';
lin.style.fill = 'none';
div.style = 'font-size: 32px; overflow: auto;'

txt.setAttribute('x', w / 4);
txt.setAttribute('y', 150);
txt.setAttribute('width', 300);
txt.setAttribute('height', 300);

svg.appendChild(lin);
svg.appendChild(txt);
txt.appendChild(div);
document.body.appendChild(svg);

export default
{
    pts: "",
    begin()
    {
        this.pts = "";
        this.txt = "";
    },
    add(frq, amp, len)
    {
        this.pts += `${frq * (w / len)}, ${h - 30 - amp * (h * 0.9)} `
    },
    end()
    {
        lin.setAttribute('points', this.pts);
        div.textContent = this.txt;
    },
    text(str)
    {
        this.txt += `${str}\n`;
    }
}