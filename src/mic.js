/**
 * begin a new microphone input session and run `fn`
 * on the raw audio data as its recorded
 * 
 * @param {Number} len length of audio buffer
 * @param {function(Float32Array, number)} fn audio processr; arg1 is sample rate
 */
const microphone = (len, fn) =>
{
    navigator.mediaDevices
        .getUserMedia({ audio: true, video: false })
        .then(stream =>
        {
            /** @type {AudioContext} */
            const ctx = new (window.AudioContext || window.webkitAudioContext)();

            const input = ctx.createMediaStreamSource(stream);
            /** @type {ScriptProcessorNode} */
            const recorder = (ctx.createScriptProcessor || ctx.createJavaScriptNode).call(ctx, len, 1, 1);
            // audio processor
            recorder.onaudioprocess = e =>
            {
                // get raw audio data
                const pcm = e.inputBuffer.getChannelData(0);

                // run processor
                fn(pcm, ctx.sampleRate);
            }

            input.connect(recorder);
            recorder.connect(ctx.destination);
        })
        .catch(e => console.error(e));
}
export default microphone;