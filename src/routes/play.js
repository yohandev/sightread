import m from 'mithril'

import microphone from '../mic';
import crate from '../crate';
import draw from '../draw';
import notes from '../notes';

const align = (note) =>
{
    switch (note.charAt(0))
    {
        case 'G': return 3;
        case 'F': return 11;
        case 'E': return 18;
        case 'D': return 25;
        case 'C': return 32;
        case 'B': return 39;
        case 'A': return 46;
    }
}

const play =
{
    view: () =>
    (
        <div class='page'>
            {/* play an A: */}
            {/* { play.state['A4'] ? (<h1>You played an A4! Not so stupid after all</h1>) : null } */}
            <div id='composer'>
                <img src={require('../../res/images/beethoven.png')} width='150'></img>
                <h1>5th Symphony</h1>
                <i>Beethoven</i>
            </div>
            <svg id='notes-container'>'
                { draw.treble }
                {
                    play.state.notes.map(note =>
                        <g class='notes-down' style={`transform: translate(1.25em, ${align(note.note)}%)`}>
                            { draw.note_down }
                        </g>
                )}
            </svg>
        </div>
    ),
    state:
    {
        notes: [{note: 'A4', played: false}, {note: 'G4', played: false}],
        done: false
    }
}

// PCM buffer in wasm memory
const buf = crate.alloc['f32[]'](2048);

// begin microphone listen
microphone.listen(buf.len, x =>
{
    // upload to wasm
    buf.f32.set(x);

    let done = true;
    play.state.notes.forEach(note =>
    {
        if (!note.played)
        {
    
        done = false;
        const freq = notes[note.note];

        note.played = crate.freqAmount(buf, freq, 10) * 100 > 2;
        }
    })

    if (done)
    {
        console.log('YAY');
        const keys = Object.keys(notes);

        play.state.notes = [];
        for (let i = 0; i < Math.random() * 5; i++)
        {
            play.state.notes.push({note: keys[Math.round(keys.length * Math.random())], played: false})
        }
        console.log(play.state.notes)
        //play.state.notes.forEach(note => note.played = false);

        m.redraw();
    }
})

export default play;