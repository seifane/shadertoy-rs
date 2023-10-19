use gfx::{Encoder, Factory, texture};
use gfx::format::{R8_G8_B8_A8, Rgba8, SurfaceTyped, Vec4};
use gfx::handle::ShaderResourceView;
use gfx::handle::Sampler;
use gfx::texture::{FilterMethod, Mipmap, WrapMode};
use gfx::{traits::FactoryExt, Device};
use gfx::memory::{Bind, Usage};
use gfx_device_gl::{CommandBuffer, Resources};
use audio::player::AudioPlayer;
use error;

pub trait Channel
{
    fn invalidated(&self) -> bool;
}

pub struct ImageChannel<R> where R: gfx::Resources {
    shader_resource_view: ShaderResourceView<R, Vec4<f32>>,
    sampler: Sampler<R>,
}

impl<R: gfx::Resources> ImageChannel<R> {
    pub fn try_from_path<F: gfx::Factory<R>>(mut factory: F, path: String, filter: FilterMethod, wrap: WrapMode)
                                             -> error::Result<ImageChannel<R>>
    {
        let img = image::open(&path)?.flipv().to_rgba();

        let (w, h) = img.dimensions();
        let (_, view) =
            factory.create_texture_immutable_u8::<Rgba8>(
                gfx::texture::Kind::D2(w as u16, h as u16, gfx::texture::AaMode::Single),
                Mipmap::Allocated,
                &[&img],
            )?;

        Ok(ImageChannel {
            shader_resource_view: view,
            sampler: factory.create_sampler(gfx::texture::SamplerInfo::new(filter, wrap)),
        })
    }

    pub fn draw<C: gfx::CommandBuffer<R> + Send>(&mut self, encoder: &mut Encoder<R, C>)
                                                 -> (ShaderResourceView<R, Vec4<f32>>, Sampler<R>)
    {
        if self.sampler.get_info().filter != FilterMethod::Scale && self.sampler.get_info().filter != FilterMethod::Bilinear {
            encoder.generate_mipmap(&self.shader_resource_view)
        }
        return (
            self.shader_resource_view.clone(),
            self.sampler.clone()
        );
    }
}

pub struct SoundChannel<R, F> where R: gfx::Resources, F: Factory<R> {
    sampler: Sampler<R>,
    factory: F,
    audio_player: AudioPlayer,
    buffer: Vec<u8>,
}

impl<R: gfx::Resources, F: Factory<R>> SoundChannel<R, F> {
    pub fn try_from(mut factory: F, audio_path: String) -> SoundChannel<R, F> {
        SoundChannel {
            sampler: factory.create_sampler(gfx::texture::SamplerInfo::new(FilterMethod::Mipmap, WrapMode::Tile)),
            factory,
            audio_player: AudioPlayer::new(audio_path),
            buffer: Vec::with_capacity(512 * 2 * 4),
        }
    }

    pub fn play(&mut self)
    {
        self.audio_player.play();
    }

    pub fn draw<C: gfx::CommandBuffer<R> + Send>(&mut self, encoder: &mut Encoder<R, C>) -> (ShaderResourceView<R, Vec4<f32>>, Sampler<R>) {
        let fft_lock = self.audio_player.ftt_data.read().unwrap();

        self.buffer.clear();

        for i in 0..1024 {
            let mut val: u8 = 0;
            if i < 512 {
                val = (fft_lock.get(i).unwrap().1.val() * 255.0) as u8;
            }
            self.buffer.push(val);
            self.buffer.push(0);
            self.buffer.push(0);
            self.buffer.push(0);
        }

        let (_, view) = self.factory.create_texture_immutable_u8::<Rgba8>(
            texture::Kind::D2(512, 2, gfx::texture::AaMode::Single),
            Mipmap::Allocated,
            &[self.buffer.as_slice()],
        ).unwrap();

        if self.sampler.get_info().filter != FilterMethod::Scale && self.sampler.get_info().filter != FilterMethod::Bilinear {
            encoder.generate_mipmap(&view)
        }

        (
            view,
            self.sampler.clone(),
        )
    }
}