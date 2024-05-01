use anyhow::bail;
use clap::Parser;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, SizedSample, SupportedStreamConfig,
};
use macros::tauri_anyhow;
use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};
use tauri::Manager;

#[derive(Parser, Debug)]
#[command(version, about = "CPAL beep example", long_about = None)]
struct Opt {
    /// The input audio device to use
    #[arg(short, long, value_name = "IN", default_value_t = String::from("default"))]
    input: String,

    /// The output audio device to use
    #[arg(short, long, value_name = "OUT", default_value_t = String::from("default"))]
    output: String,

    /// Use the JACK host
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}

struct AudioDevices {
    pub input: Device,
    pub input_config: SupportedStreamConfig,
    pub output: Device,
    pub output_config: SupportedStreamConfig,
}

macro_rules! match_sample_format {
    ($sample_format:expr, $function:expr, $setup:expr) => {
        match $sample_format {
            cpal::SampleFormat::I8 => $function::<i8>($setup),
            cpal::SampleFormat::I16 => $function::<i16>($setup),
            cpal::SampleFormat::I32 => $function::<i32>($setup),
            cpal::SampleFormat::F32 => $function::<f32>($setup),
            sample_format => panic!("Unsupported sample format '{:?}'", sample_format),
        }
    };
}

#[tauri::command]
#[tauri_anyhow]
pub async fn play_sound() -> anyhow::Result<()> {
    let setup = setup_audio_devices()?;

    // match_sample_format!(setup.output_config.sample_format(), beep, setup)
    match setup.output_config.sample_format() {
        cpal::SampleFormat::I8 => beep::<i8>(setup),
        cpal::SampleFormat::I16 => beep::<i16>(setup),
        cpal::SampleFormat::I32 => beep::<i32>(setup),
        cpal::SampleFormat::F32 => beep::<f32>(setup),
        sample_format => panic!("Unsupported sample format '{:?}'", sample_format),
    }
}

#[tauri::command]
#[tauri_anyhow]
pub async fn start_recording(app: tauri::AppHandle) -> anyhow::Result<()> {
    let setup = setup_audio_devices()?;

    match setup.input_config.sample_format() {
        cpal::SampleFormat::I8 => record::<i8>(setup, app),
        cpal::SampleFormat::I16 => record::<i16>(setup, app),
        cpal::SampleFormat::I32 => record::<i32>(setup, app),
        cpal::SampleFormat::F32 => record::<f32>(setup, app),
        sample_format => panic!("Unsupported sample format '{:?}'", sample_format),
    }
}

fn setup_audio_devices() -> anyhow::Result<AudioDevices> {
    let opt = Opt::parse();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo beep --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .expect(
                    "make sure --features jack is specified. only works on OSes where jack is available",
                )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    let input = if opt.input == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.input).unwrap_or(false))
    }
    .expect("failed to find input device");

    let output = if opt.output == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.output).unwrap_or(false))
    }
    .expect("failed to find output device");

    let output_config = output.default_output_config()?;
    let input_config = input.default_input_config()?;

    Ok(AudioDevices {
        input,
        input_config,
        output,
        output_config,
    })
}

fn beep<T>(
    AudioDevices {
        output,
        output_config,
        ..
    }: AudioDevices,
) -> anyhow::Result<()>
where
    T: SizedSample + FromSample<f32>,
{
    let output_config: &cpal::StreamConfig = &output_config.into();

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = output.build_output_stream(
        output_config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_output_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_output_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn record<T>(
    AudioDevices {
        input,
        input_config,
        ..
    }: AudioDevices,
    app: tauri::AppHandle,
) -> anyhow::Result<()>
where
    T: cpal::Sample + SizedSample + hound::Sample + FromSample<T>,
{
    // The WAV file we're recording to.
    const PATH: &str = "../assets/recorded.wav";
    let spec = hound::WavSpec {
        channels: input_config.channels() as _,
        sample_rate: input_config.sample_rate().0 as _,
        bits_per_sample: (input_config.sample_format().sample_size() * 8) as _,
        sample_format: if input_config.sample_format().is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        },
    };
    let writer = hound::WavWriter::create(PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let (tx, rx) = std::sync::mpsc::channel();

    let stream = input.build_input_stream(
        &input_config.clone().into(),
        move |data, _: &_| write_input_data::<T, T>(data, &writer_2),
        err_fn,
        None,
    )?;

    stream.play()?;

    app.once_any("stop_recording", move |_| {
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();
    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize().unwrap();
    println!("Recording {} complete!", PATH);

    Ok(())
}

fn write_input_data<T, U>(
    input: &[T],
    writer: &Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>,
) where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
