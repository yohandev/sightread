import m from 'mithril'

import microphone from '../mic';
import crate from '../crate';
import draw from '../draw';

const play =
{
    view: () =>
    (
        <div class='page'>
            {/* play an A: */}
            {/* { play.state['A4'] ? (<h1>You played an A4! Not so stupid after all</h1>) : null } */}
            <svg id='notes-container'>'
                { draw.treble }
                <g class='notes-down' style={`transform: translate(1.25em, ${3}%)`}>
                    { draw.note_down }
                </g>
            </svg>
        </div>
    ),
    state:
    {
        'A4': false,
    }
}

// PCM buffer in wasm memory
const buf = crate.alloc['f32[]'](2048);

// begin microphone listen
microphone.listen(buf.len, x =>
{
    // upload to wasm
    buf.f32.set(x);

    // find requency
    const freq = 440; // A4

    // played the note?
    const played = crate.freqAmount(buf, freq, 10) * 100 > 2;
    if (played)
    {
        console.log(`played note@${freq}hz!`);
    }
    if (played != play.state['A4'])
    {
        play.state['A4'] = played;

        m.redraw();
    }
})

export default play;