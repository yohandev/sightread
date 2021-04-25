import microphone from './mic';
import crate from './crate';
import draw from './draw';
import chords from './chords';

const buf = crate.alloc['f32[]'](2048);


microphone.listen(buf.len, x =>
{
    // upload to wasm
    buf.f32.set(x);

    // find requency
    const freq = 440; // A4

    let { score, phase } = crate.freqAmount(buf, freq, 10);
    
    console.log(`${freq}hz: ${score * 100}`);

    draw(x, freq, phase);
})
