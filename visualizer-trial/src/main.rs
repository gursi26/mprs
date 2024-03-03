#![allow(unused_imports, dead_code, unused)]

use itertools::Itertools;
use rodio::cpal::PlayStreamError;
use rodio::{source::Source, Decoder, OutputStream};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::fs::File;
use std::io::{stdout, BufReader, Write};
use std::usize;
// use macroquad::prelude::*;
use raylib::prelude::*;
use std::{thread, time};

const FREQUENCY_RESOLUTION: u32 = 100;
const FREQ_WINDOW_LOW: f32 = 0.0;
const FREQ_WINDOW_HIGH: f32 = 5000.0;
const MONO_SCALING_FACTOR: i32 = 10;
const FFT_FPS: u32 = 15;
const RENDERING_FPS: u32 = FFT_FPS * 5;
const FFT_WINDOW: i32 =
    ((256 as u64 / 107 as u64) * FREQUENCY_RESOLUTION as u64).next_power_of_two() as i32;

// const AUDIO_PATH: &str = "/Users/gursi/Desktop/60 BPM - Metronome-[AudioTrimmer.com].mp3";
// const AUDIO_PATH: &str = "/Users/gursi/Desktop/gurenge.mp3";
const AUDIO_PATH: &str = "/Users/gursi/mprs-music/rn/ハルジオン.mp3";

fn main() {
    let mut fft = get_fft().into_iter().peekable();
    let mut i = 0;

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("audio-visualizer-trial")
        .build();
    rl.set_target_fps(RENDERING_FPS);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(AUDIO_PATH).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());

    let num_frame_gen = RENDERING_FPS / FFT_FPS;
    let mut fft_chunk: Vec<f32> = Vec::new();

    while !rl.window_should_close() && fft.peek().is_some() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if i as u32 % num_frame_gen == 0 {
            fft_chunk = fft.next().unwrap();
        } else {
            let next_chunk = fft.peek().unwrap();
            fft_chunk = interpolate_vecs(&fft_chunk, next_chunk);
        }

        let (h, w) = (d.get_screen_height(), d.get_screen_width());
        let bar_start_idxs = (0..((w as i32) + 1))
            .step_by(w as usize / fft_chunk.len() as usize)
            .collect::<Vec<i32>>();

        for j in 0..(fft_chunk.len() as usize) {
            let curr_fft_value = (fft_chunk[j] * h as f32 * 2.0) as i32;
            let (start_x_id, end_x_id) = (bar_start_idxs[j], bar_start_idxs[j + 1]);
            d.draw_rectangle(
                start_x_id,
                h - curr_fft_value,
                end_x_id - start_x_id,
                curr_fft_value,
                Color::GREEN,
            );
        }
        i += 1;
    }
}

fn interpolate_vecs(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    v1.iter()
        .zip(v2.iter())
        .map(|(x, y)| (x.clone() + y.clone()) / 2.0)
        .collect::<Vec<f32>>()
}

fn get_fft() -> Vec<Vec<f32>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(AUDIO_PATH).unwrap());
    let source = Decoder::new(file).unwrap();

    let n_channels = source.channels() as i32;
    let sample_rate = source.sample_rate() as i32;
    println!("{}, {}", n_channels, sample_rate);

    println!("Collecting...");

    // Collecting takes too long
    let sample_vec: Vec<i32> = source.map(|x| x as i32).collect();
    println!("{}", sample_vec.len());

    let mut output_vec = Vec::new();

    let (mut min, mut max): (f32, f32) = (100.0, 0.0);
    println!("Computing FFT...");

    for x in sample_vec.chunks((sample_rate * n_channels) as usize / FFT_FPS as usize) {
        let frame = &x[..(FFT_WINDOW as usize * n_channels as usize)];
        let mut samples = [0.0; FFT_WINDOW as usize];
        for (i, stereo) in frame.chunks(n_channels as usize).enumerate() {
            samples[i] = stereo
                .iter()
                .map(|x| x.clone() as f32 / (n_channels * MONO_SCALING_FACTOR) as f32)
                .sum::<f32>();
        }

        let hann_window = hann_window(&samples);
        let spectrum_hann_window = samples_fft_to_spectrum(
            &hann_window,
            sample_rate as u32,
            FrequencyLimit::Range(FREQ_WINDOW_LOW, FREQ_WINDOW_HIGH),
            Some(&divide_by_N_sqrt),
        )
        .unwrap();

        let curr_vec = spectrum_hann_window
            .data()
            .into_iter()
            .map(|(f, fval)| fval.val())
            .collect::<Vec<f32>>();

        for val in curr_vec.iter() {
            max = max.max(*val);
            min = min.min(*val);
        }
        output_vec.push(curr_vec.clone());
    }

    let scale = 1.0 / (max - min);
    for j in 0..output_vec.len() {
        for k in 0..output_vec[j].len() {
            output_vec[j][k] -= min;
            output_vec[j][k] *= scale;
        }
    }

    println!("Min: {}, Max: {}", min, max);
    println!("{}, {}", output_vec.len(), output_vec[0].len());
    output_vec
}
