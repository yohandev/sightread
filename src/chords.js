import { Midi } from '@tonejs/midi';

Midi
    .fromUrl(require('../res/songs/Megalovania.mid'))
    .then(midi =>
    {
        midi.tracks.forEach(track => 
        {
            const name = midi.name
            const notes = track.notes
            
            notes.forEach(note =>
            {

            })
        })
        console.log(midi.tracks);
    })
