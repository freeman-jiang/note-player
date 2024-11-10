# simple synth

This is a simple sine wave synth written in Rust. It plays notes by reading from a JSON file that describes the notes and duration of each song.

Fully supports [Scientific Pitch Notation (SPN)](https://en.wikipedia.org/wiki/Scientific_pitch_notation) (eg. `C#4`, `A5`, `Bb3`, etc.) in equal temperament.

## Usage

Try playing one of the songs in the `songs` directory:

```
cargo run --release < songs/greensleeves.json
```

### Examples

**Scales: Eb Major, B Harmonic Minor, C Blues Scale**

https://github.com/user-attachments/assets/10b7d0e0-3d73-40dd-b7ae-fd97dccad0e7


**Jingle Bells**

https://github.com/user-attachments/assets/296df5f6-cef7-4826-8a47-a1c611b760e3
