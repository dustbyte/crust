use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

use crate::context::Context;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase >= 0.0 && self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Buzzer {
    device: AudioDevice<SquareWave>,
}

impl Buzzer {
    pub fn new(ctx: &Context) -> Self {
        let audio_subsystem = ctx.as_raw().audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.10,
            }
        }).unwrap();

        Self {
            device: device,
        }
    }

    pub fn play(&mut self) {
        self.device.resume();
    }

    pub fn pause(&mut self) {
        self.device.pause();
    }
}