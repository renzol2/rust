# wavetable synth

an exploration of basic wavetable synthesis in rust, using this article as a start: <https://www.thewolfsound.com/sound-synthesis/wavetable-synth-in-rust/>

some ideas on extending:

- store output in .wav file
- use different interpolation methods
- allow adjustable gain through CLI args
- reduce wavetable synthesis noise:
  - increase size of wave table
  - increase order of interpolation (linear interpolation)
  - basically, time vs. memory
- implement fade-in & fade-out
- allow switching of wavetables through CLI args
  - allow wavetable concatenation through CLI args
- allow programming of notes/frequencies and rhythms through CLI args
  - use crates to facilitate musical parameters
- add proper duration and ADSR envelope
- create design doc with set features and move to separate repository
