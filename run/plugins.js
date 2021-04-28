// wasm plugin
const wasm = 
{
    name: 'wasm',
    setup(build)
    {
        const path = require('path');
        const fs = require('fs');

        build.onResolve({ filter: /\.wasm$/ }, args =>
        {
            // import binary to js virtual module
            if (args.namespace === 'wasm-stub')
            {
                return { path: args.path, namespace: 'wasm-binary' }
            }
            // path not resolvable
            if (args.resolveDir === '') { return }
            // use absolute path
            let absPath = path.isAbsolute(args.path) ? args.path : path.join(args.resolveDir, args.path);

            return { path: absPath, namespace: 'wasm-stub' }
        })
        // generate virtual js module under 'wasm-stub' namespace
        build.onLoad({ filter: /.*/, namespace: 'wasm-stub' }, async (args) =>
        ({
            contents:
            `
            import wasm from ${JSON.stringify(args.path)}

            export default (imports) => WebAssembly
                .instantiate(wasm, imports)
                .then(result => result.instance.exports)
            `,
        }))
        // actual bytes of wasm module
        build.onLoad({ filter: /.*/, namespace: 'wasm-binary' }, async (args) =>
        ({
            contents: await fs.promises.readFile(args.path),
            loader: 'binary',
        }))
    }
}

module.exports = [ wasm ]