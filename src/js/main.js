import wasm from '../rs/mod'

const main = async () =>
{
    await wasm.load()

    let arr = wasm.alloc('f32[]', 30);

    wasm.dealloc(arr);
}

main();