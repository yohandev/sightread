;(async () =>
{
    const { spawn } = require('child_process')

    const port = +process.env.PORT || 1234;
    const cwd = require('process').cwd();

    // responsible for bundling js/ts/jsx/tsx
    const esbuild = await (require('esbuild').build(
    {
        entryPoints: ['src/js/main.js'],
        bundle: true,
        minify: true,
        sourcemap: true,
        incremental: true,

        target: ['chrome58', 'firefox57', 'safari11', 'edge16'],

        plugins: [...require('./plugins')],
        
        outfile: 'pub/out.js'
    
    }))
    // responsible for building rs -> wasm
    const rsbuild =
    {
        opt: [
            'build',
            '--target', 'wasm32-unknown-unknown',
            '--release',
            '--manifest-path=src/rs/Cargo.toml',
            '--target-dir=.cache/rs-target'
        ],
        gc: [
            ".cache/rs-target/wasm32-unknown-unknown/release/sightread.wasm",
            "src/rs/mod.wasm"
        ],
        rebuild()
        {
            return Promise.resolve(spawn('cargo', this.opt, { cwd: cwd, stdio: 'inherit' })
                .on('message', console.log)
                .on('error', console.error)
                .on('exit', _ =>
                {                    
                    // strip output wasm and recompile
                    spawn('wasm-gc', this.gc, { cmd: cwd, stdio: 'inherit' })
                        .on('error', console.error)
                        .on('exit', _ => esbuild.rebuild());
                })
            )
        }
    }
    require('chokidar')
        // watch for (react) javascript and typescript
        .watch('src/**/*.{ts,tsx,js,jsx,rs}', { interval: 0 })
        // rebuilt incrementally
        .on('all', (_, path) =>
        {
            // bundle or compile?
            const builder = path.slice(-3) == '.rs' ? rsbuild : esbuild;
            // last time built
            const time = new Date()
                .toTimeString()
                .split(' ')[0];

            console.clear();
            builder
                .rebuild()
                .catch(console.log);
            console.log('\x1b[36m%s\x1b[0m', `Serving at http://127.0.0.1:${port}`, `(${time})`);
        })
        
    require('live-server').start(
    {
        // open locally on start
        open: true,
        // port fallback
        port: port,
        // serve pkg/index.html
        root: 'pub',
        // shut up
        logLevel: 0,
    })
})()