use rodio::{OutputStream, Sink, Source};
use serde::Deserialize;
use std::io::Read;
use std::time::Duration;
use std::{f32::consts::PI, io};

const SAMPLE_RATE: f32 = 44100.0;
const A4_FREQ: f32 = 440.0;
const OCTAVE_SEMITONES: i32 = 12;

#[derive(Deserialize)]
struct Note {
    // Note in the form of "C4", "A2", etc.
    note: String,
    duration: f32,
}

impl Note {
    /// f = 2^{n/12}*440, where n is the number of semitones above or below A4
    fn frequency(self: &Self) -> f32 {
        if self.note.len() != 2 {
            panic!("Invalid note: {}", &self.note);
        }

        let note = self.note.chars().nth(0).unwrap();

        // Relative octave to A4
        let relative_octave = self.note.chars().nth(1).unwrap().to_digit(10).unwrap() as i32 - 4;

        // Semitones that A4/B4/C4/etc is from A4
        let n: i32 = match note {
            'A' => 0,
            'B' => 2,
            'C' => -9,
            'D' => -7,
            'E' => -5,
            'F' => -4,
            'G' => -2,
            _ => {
                panic!("Invalid note: {}", note);
            }
        };

        let semitones_from_a4 = n + relative_octave * OCTAVE_SEMITONES;

        let freq = 2.0_f32.powf(semitones_from_a4 as f32 / 12.0) * A4_FREQ;
        return freq;
    }
}

// Simple sine wave generator
struct SineWave {
    /// Note frequency in Hz
    freq: f32,
    /// Duration in seconds
    duration: f32,
    current_sample: f32,
    total_samples: f32,
}

impl SineWave {
    fn new(freq: f32, duration: f32) -> Self {
        Self {
            freq,
            duration,
            current_sample: 0.0,
            total_samples: duration * SAMPLE_RATE,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current_sample >= self.total_samples {
            return None; // End of audio
        }

        let t = self.current_sample / self.sample_rate() as f32; // time in seconds
        let output = (2.0 * PI * self.freq * t).sin();
        self.current_sample += 1.0;
        Some(output * 0.5) // Reduce amplitude to 0.5 to avoid clipping
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // mono audio
    }

    fn sample_rate(&self) -> u32 {
        44100 // 44.1 kHz sample rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.duration))
    }
}

fn main() {
    // Read JSON input
    println!("Enter notes JSON array:");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let notes: Vec<Note> = serde_json::from_str(&input).unwrap();

    // Set up audio output
    let (_stream, output_stream_handle) = OutputStream::try_default().unwrap();
    let output_sink = Sink::try_new(&output_stream_handle).unwrap();

    // Play each note
    for note in notes {
        let freq = note.frequency();
        output_sink.append(SineWave::new(freq, note.duration));
    }

    output_sink.sleep_until_end();
}
