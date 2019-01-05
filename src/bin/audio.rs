use portaudio as pa;
use std::sync::{Arc, Mutex};
use toy_pcm::{hz, Playback, Wave};

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const CHANNELS: i32 = 1;

struct Context<W>
where
    W: Wave,
{
    sound: Playback<W>,
}

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;

    println!("PortAudio:");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", pa.host_api_count()?);

    let default_host = pa.default_host_api()?;
    println!("default host: {:#?}", pa.host_api_info(default_host));

    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    println!("Default output device info: {:#?}", &output_info);

    let settings =
        pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;

    let default_play = Playback::new(hz::rate(44_100.0).const_hz(261.626).phase().sine(), 0);
    let ctx = Arc::new(Mutex::new(Context {
        sound: default_play,
    }));

    let cb_ctx = ctx.clone();
    // A callback to pass to the non-blocking stream.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
        let mut ctx = cb_ctx.lock().unwrap();
        for out_frame in buffer {
            match ctx.sound.next() {
                Some(frame) => {
                    *out_frame = frame[0] as f32;
                }
                None => return pa::Continue,
            }
        }
        pa::Continue
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    let mut input = String::new();
    let mut end = false;

    let main_ctx = ctx.clone();
    while stream.is_active()? && !end {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match (&*input).trim() {
                "q" => {
                    println!("quit...");
                    end = true;
                }
                "c" => {
                    println!("C4");
                    let mut ctx = main_ctx.lock().unwrap();
                    ctx.sound = Playback::new(
                        hz::rate(44_100.0).const_hz(261.626).phase().sine(),
                        (SAMPLE_RATE * 1.0) as usize,
                    );
                }
                "d" => {
                    println!("D4");
                    let mut ctx = main_ctx.lock().unwrap();
                    ctx.sound = Playback::new(
                        hz::rate(44_100.0).const_hz(293.665).phase().sine(),
                        (SAMPLE_RATE * 1.0) as usize,
                    );
                }
                "e" => {
                    println!("E4");
                    let mut ctx = main_ctx.lock().unwrap();
                    ctx.sound = Playback::new(
                        hz::rate(44_100.0).const_hz(329.628).phase().sine(),
                        (SAMPLE_RATE * 1.0) as usize,
                    );
                }
                "a" => {
                    println!("A4");
                    let mut ctx = main_ctx.lock().unwrap();
                    ctx.sound = Playback::new(
                        hz::rate(44_100.0).const_hz(440.0).phase().sine(),
                        (SAMPLE_RATE * 1.0) as usize,
                    );
                }
                "c+a" => {
                    println!("C4+A4");
                    let mut ctx = main_ctx.lock().unwrap();
                    ctx.sound = Playback::new(
                        hz::rate(44_100.0).const_hz(261.626 + 440.0).phase().sine(),
                        (SAMPLE_RATE * 1.0) as usize,
                    );
                }
                _ => println!("unknwon>>{}", (&*input).trim()),
            },
            Err(error) => println!("error: {}", error),
        }
        input.clear();
    }

    println!("stop...");
    r#try!(stream.stop());
    println!("stoped");

    println!("close...");
    r#try!(stream.close());
    println!("closed...");

    Ok(())
}
