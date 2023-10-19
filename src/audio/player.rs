use std::error::Error;
use std::fs::{File};
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;
use cpal::{BufferSize, BuildStreamError, Device, SampleRate, Stream, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavReader;
use rb::{Producer, RB, RbConsumer, RbProducer, SpscRb};
use spectrum_analyzer::{Frequency, FrequencyLimit, FrequencyValue, samples_fft_to_spectrum};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;

fn build_stream(device: &Device, config: StreamConfig, mut reader: WavReader<BufReader<File>>, producer: Producer<f32>) -> Result<Stream, BuildStreamError> {
    println!("Applied config {:?}", config);
    device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut samples = reader.samples::<f32>().take(data.len()).map(|i| {
                i.unwrap()
            }).collect::<Vec<_>>();

            let mut index = 0;
            for sample in data.iter_mut() {
                if let Some(frame) = samples.get(index) {
                    *sample = cpal::Sample::from_sample(*frame);
                }
                index += 1;
            }

            producer.write(samples.as_slice());
        },
        move |_err| {
        },
        None,
    )
}


pub struct AudioPlayer {
    pub path: String,
    pub thread_handle: Option<JoinHandle<()>>,
    pub is_running: AtomicBool,
    pub ftt_data: Arc<RwLock<Vec<(Frequency, FrequencyValue)>>>,
}

impl AudioPlayer {
    pub fn new(path: String) -> AudioPlayer
    {
        AudioPlayer {
            path,
            thread_handle: None,
            is_running: AtomicBool::from(false),
            ftt_data: Arc::new(RwLock::new(Vec::new()))
        }
    }

    pub fn play(&mut self) -> Result<(), Box<dyn Error>>
    {
        self.is_running.store(true, Ordering::SeqCst);

        let path = self.path.clone();
        let shared = self.ftt_data.clone();
        let handle = std::thread::spawn(move || {
            let reader = WavReader::open(path).unwrap();

            let host = cpal::default_host();
            let device = host.default_output_device().expect("No out");
            let config = device.default_output_config().unwrap();

            println!("{:?}", config);

            let custom_config = StreamConfig {
                channels: 2,
                sample_rate: SampleRate(48000),
                buffer_size: BufferSize::Default,
            };

            let rb: SpscRb<f32> = SpscRb::new(48000);
            let (producer, consumer) = (rb.producer(), rb.consumer());

            let stream = build_stream(&device, custom_config, reader, producer).unwrap();
            stream.play().unwrap();

            loop {
                std::thread::sleep(Duration::from_millis(20));
                let mut samples: [f32; 2048] = [0.0 as f32; 2048];
                if let Err(e) = consumer.read(&mut samples){
                    println!("{:?}", e);
                    continue;
                }

                let hann_window = hann_window(&samples);
                // calc spectrum
                let spectrum_hann_window = samples_fft_to_spectrum(
                    // (windowed) samples
                    &hann_window,
                    // sampling rate
                    48000,
                    // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                    FrequencyLimit::All,
                    // optional scale
                    Some(&divide_by_N_sqrt),
                ).unwrap();


                println!("len {}", spectrum_hann_window.data().len());
                let mut lock = shared.write().unwrap();
                lock.clear();
                for datum in spectrum_hann_window.data().iter() {
                    lock.push(*datum);
                }
                drop(lock);
            }
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
    }
}