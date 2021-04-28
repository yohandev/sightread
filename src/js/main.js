import wasm from '../rs/mod'

const main = async () =>
{
    await wasm.load()

    console.log(`4 + 2 = ${wasm.add(4, 2)}`)
}

main();