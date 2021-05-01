# note classification engine
## approach 01 - known harmonics removal
In frequency domain, begin with the loudest amplitude.
Then, determine its (approximately) integer multiple
overtones and record their amplitude, then, using known
relative amplitudes, determine the minimum absolute
loudless of the note. mark that note and remove its
fundamental and harmonic frequencies, then repeat the
process
## approach 02 - neural network
feed freqeuntry spectrum to (recurrent?) neural network
with output vector spanning 88 keys' current state, on or off