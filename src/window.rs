use gfx_device_gl::{Factory, Resources, CommandBuffer};
use image::RgbaImage;
use piston_window::{TextureContext, clear, Event};

pub struct Window{
    window : piston_window::PistonWindow,
    texture_context: TextureContext<Factory, Resources, CommandBuffer>
}

impl Window{
    pub fn new(width: u32, height: u32) -> Self{
        let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Raytracer", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_| panic!("Could not create window!"));
        let texture_context = piston_window::TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };
        Window{window, texture_context}
    }
    pub fn show_image(&mut self, image : &RgbaImage, e : Event){
        self.window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            let texture: piston_window::G2dTexture = piston_window::Texture::from_image(
                &mut self.texture_context,
                image,
                &piston_window::TextureSettings::new(),
            )
            .unwrap();
            piston_window::image(&texture, c.transform, g);
        });
    }

    pub fn next(&mut self) -> Option<Event>{
        self.window.next()
    }
}

