import microphone from './mic';
import crate from './crate';

const buf = crate.alloc['f32[]'](1024);

microphone.listen(buf.len, x =>
{
    // upload to wasm
    buf.f32.set(x);
})
