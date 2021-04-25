/**
 * abstraction over microphone input
 */
const microphone = 
{
    inner:
    {
        /** @type {AudioContext} */
        context: null,
        /** @type {MediaStreamAudioSourceNode} */
        source: null,
    },
    /**
     * initialize microphone
     */
    init: () =>
    {
        if (microphone.inner.context != null && microphone.inner.source != null)
        {
            return new Promise();
        }
        return navigator.mediaDevices
            .getUserMedia({ audio: true, video: false })
            .then(stream =>
            {
                /** @type {AudioContext} */
                const context = new (window.AudioContext || window.webkitAudioContext)();
                /** @type {MediaStreamAudioSourceNode} */
                const source = context.createMediaStreamSource(stream);

                microphone.inner.context = context;
                microphone.inner.source = source;
            })
            .catch(e => console.error(e))
    },
    /**
     * add a new audio processor, running the given function on raw
     * audio samples
     * @param {Number} len length of sample buffer
     * @param {function(Float32Array)} fn function to run on samples
     */
    listen: (len, fn) =>
    {
        microphone
            .init()
            .then(_ =>
        {
            /** @type {AudioContext} */
            const context = microphone.inner.context;
            /** @type {MediaStreamAudioSourceNode} */
            const source = microphone.inner.source;
            /** @type {ScriptProcessorNode} */
            const recorder = (context.createScriptProcessor || context.createJavaScriptNode).call(context, len, 1, 1);

            // run processor on raw audio samples
            recorder.onaudioprocess = e => fn(e.inputBuffer.getChannelData(0));
            // link new recorder
            source.connect(recorder);
            recorder.connect(context.destination);
        })
    },
    /** get the sample rate, in hertz */
    sampleRate: () =>
    {
        return microphone.inner.context.sampleRate
    }
}
export default microphone;