/**
 * creates a new audio processor, working with `samples`
 * samples of PCM data from microphone audio input
 * 
 * @param {number} samples number of samples desired at once
 * @param {function(Float32Array, number)} processor audio processor(samples, sampling frequency)
 */
export default async (samples, processor) =>
{
    /** @type {AudioContext} */
    const context = new (window.AudioContext || window.webkitAudioContext)();
    /** @type {MediaStreamAudioSourceNode} */
    const source = context
        .createMediaStreamSource(await navigator.mediaDevices
            .getUserMedia({ audio: true, video: false })
        );
    /** @type {ScriptProcessorNode} */
    const recorder = (context.createScriptProcessor || context.createJavaScriptNode).call(context, samples, 1, 1);
    
    // run processor on raw audio samples
    recorder.onaudioprocess = e => processor(e.inputBuffer.getChannelData(0), context.sampleRate);
    // link new recorder
    source.connect(recorder);
    recorder.connect(context.destination);
}