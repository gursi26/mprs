#![allow(unused_imports, dead_code, unused)]

use itertools::Itertools;
use rodio::cpal::PlayStreamError;
use rodio::{source::Source, Decoder, OutputStream};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::fs::File;
use std::io::{self, stdout, BufReader, Write};
use std::usize;
// use macroquad::prelude::*;
use raylib::prelude::*;
use std::{thread, time};

const FREQUENCY_RESOLUTION: u32 = 100;
const FREQ_WINDOW_LOW: f32 = 0.0;
const FREQ_WINDOW_HIGH: f32 = 5000.0;
const MONO_SCALING_FACTOR: i32 = 10;
const FFT_FPS: u32 = 15;
const RENDERING_FPS: u32 = FFT_FPS * 4;
const FFT_WINDOW: i32 =
    ((256 as u64 / 107 as u64) * FREQUENCY_RESOLUTION as u64).next_power_of_two() as i32;

// const AUDIO_PATH: &str = "/Users/gursi/Desktop/60 BPM - Metronome-[AudioTrimmer.com].mp3";
const AUDIO_PATH: &str = "/Users/gursi/Desktop/gurenge.mp3";

fn main() {
    let mut fft = get_fft();
    write_to_binary_file("/Users/gursi/Desktop/file.bin", &fft);

    for c in fft.iter_mut() {
        let mut reversed = c.clone();
        reversed.reverse();
        reversed.append(c);
        *c = reversed;
    }

    let mut fft = fft.into_iter().peekable();
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

        let mut new_chunk = fft_chunk.clone();
        new_chunk = space_interpolate(new_chunk);

        let (h, w) = (d.get_screen_height(), d.get_screen_width());
        let bar_start_idxs = (0..((w as i32) + 1))
            .step_by(w as usize / new_chunk.len() as usize)
            .collect::<Vec<i32>>();

        for j in 0..(new_chunk.len() as usize) {
            let curr_fft_value = (new_chunk[j] * h as f32) as i32;
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

fn space_interpolate(v: Vec<f32>) -> Vec<f32> {
    let mut output = Vec::new();
    for i in 0..(v.len() - 1) {
        output.push(v[i]);
        output.push((v[i] + v[i + 1]) / 2.0);
    }
    output
}

fn write_to_binary_file(filepath: &str, data: &Vec<Vec<f32>>) -> io::Result<()> {
    let mut file = File::create(filepath)?;

    for row in data {
        // Write each row to the file
        file.write_all(unsafe {
            // This assumes that the layout of Vec<f32> is contiguous in memory
            std::slice::from_raw_parts(row.as_ptr() as *const u8, row.len() * std::mem::size_of::<f32>())
        })?;
    }

    Ok(())
}

fn get_fft() -> Vec<Vec<f32>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(AUDIO_PATH).unwrap());
    let mut source = Decoder::new(file).unwrap();

    let n_channels = source.channels() as i32;
    let sample_rate = source.sample_rate() as i32;

    let mut source = source.peekable();

    println!("Channels: {}, Sample Rate: {}", n_channels, sample_rate);
    println!("Computing FFT...");

    let step_amount = ((sample_rate * n_channels) as usize / FFT_FPS as usize) as i32 - FFT_WINDOW;
    let (mut min, mut max): (f32, f32) = (100.0, 0.0);
    let mut output_vec = Vec::new();

    while source.peek().is_some() {
        let mut frame = Vec::new();
        for _ in 0..FFT_WINDOW {
            frame.push(source.next().unwrap())
        }

        for _ in 0..step_amount {
            source.next();
        }

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
            let v = output_vec[j][k];
            output_vec[j][k] = (v - min) * scale;
            if output_vec[j][k] < 0.3 {
                output_vec[j][k] *= 3.0;
            } else if output_vec[j][k] < 0.5 {
                output_vec[j][k] *= 1.5;
            }
        }
    }
    output_vec
}
