# simple synth

This is a simple sine wave synth written in Rust. It plays notes by reading from a JSON file that describes the notes and duration of each song.

Fully supports [Scientific Pitch Notation (SPN)](https://en.wikipedia.org/wiki/Scientific_pitch_notation) (eg. `C#4`, `A5`, `Bb3`, etc.) in equal temperament.

## Usage

Try playing one of the songs in the `songs` directory:

```
cargo run --release < songs/greensleeves.json
```
