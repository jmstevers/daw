use anyhow::{bail, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig, SupportedStreamConfig,
};
use macros::tauri_anyhow;
use serde::{Deserialize, Serialize};
use specta::{specta, Type};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{Manager, State};
use tauri_specta::Event;

pub struct AudioDevices {
    pub host: RwLock<Host>,
    pub input: RwLock<Device>,
    pub output: RwLock<Device>,
    pub input_config: RwLock<SupportedStreamConfig>,
    pub output_config: RwLock<SupportedStreamConfig>,
}

impl Default for AudioDevices {
    fn default() -> Self {
        let host = cpal::default_host();
        let input = host
            .default_input_device()
            .expect("no input device available");
        let output = host
            .default_output_device()
            .expect("no output device available");
        let input_config = input
            .default_input_config()
            .expect("no input config available");
        let output_config = output
            .default_output_config()
            .expect("no output config available");

        Self {
            host: host.into(),
            input: input.into(),
            output: output.into(),
            input_config: input_config.into(),
            output_config: output_config.into(),
        }
    }
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
pub async fn beep(audio_devices: State<AudioDevices>) -> Result<()> {
    let device = audio_devices.output.read().unwrap().clone();
    let config: StreamConfig = audio_devices.output_config.read().unwrap().clone().into();

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0.0;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin() / 10.0
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        &config,
        move |data, _| {
            // write_output_data(data, channels, &mut next_value)
            for frame in data.chunks_mut(channels) {
                let value = next_value();
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct StopRecording(());

#[tauri::command]
#[specta]
#[tauri_anyhow]
pub async fn record(app: tauri::AppHandle, audio_devices: State<AudioDevices>) -> Result<()> {
    let device = audio_devices.input.read().unwrap().clone();
    let config = audio_devices.input_config.read().unwrap().clone();

    println!("{:?}", config.channels());

    // The WAV file we're recording to.
    const PATH: &str = "../assets/recorded.wav";
    let spec = hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: hound::SampleFormat::Float,
    };
    let writer = hound::WavWriter::create(PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();
    let err_fn = move |err| {
        panic!("an error occurred on stream: {}", err);
    };

    let stream = device
        .build_input_stream(
            &config.clone().into(),
            move |data: &[f32], _| {
                if let Ok(mut guard) = writer_2.try_lock() {
                    if let Some(writer_2) = guard.as_mut() {
                        for &sample in data.iter() {
                            writer_2.write_sample(sample).ok();
                        }
                    }
                }
            },
            err_fn,
            None,
        )
        .expect("failed to build input stream");

    stream.play()?;

    let (tx, rx) = std::sync::mpsc::channel();

    StopRecording::once_any(&app, move |_| {
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize().unwrap();
    println!("Recording {} complete!", PATH);

    Ok(())
}

#[derive(Serialize, Type)]
pub struct ValueLabel {
    pub value: String,
    pub label: String,
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
pub fn get_input_devices(audio_devices: State<AudioDevices>) -> Result<Vec<ValueLabel>> {
    Ok(audio_devices
        .host
        .read()
        .unwrap()
        .input_devices()?
        .flat_map(|device| device.name())
        .map(|name| ValueLabel {
            value: name.clone(),
            label: name,
        })
        .collect())
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
pub fn get_output_devices(audio_devices: State<AudioDevices>) -> Result<Vec<ValueLabel>> {
    Ok(audio_devices
        .host
        .read()
        .unwrap()
        .output_devices()?
        .flat_map(|device| device.name())
        .map(|name| ValueLabel {
            value: name.clone(),
            label: name,
        })
        .collect())
}

#[tauri::command]
#[specta]
#[tauri_anyhow]
pub fn get_current_output_device(audio_devices: State<AudioDevices>) -> Result<ValueLabel> {
    let name = audio_devices.output.read().unwrap().name()?;
    Ok(ValueLabel {
        value: name.clone(),
        label: name,
    })
}
