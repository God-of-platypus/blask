use raylib::{ffi::LoadTextureFromImage, prelude::*};

/// Handle the window creation for the Emulator.
///
/// It contains all the Raylib structure used to create a Window and interact with video buffer.
pub struct Window {
    handle: RaylibHandle,
    thread: RaylibThread,
    video: Texture2D,
}

impl Window {
    /// Create a new Window while initializing Raylib handles.
    pub fn new() -> Self {
        let (rl, thread) = raylib::init()
            .resizable()
            .size(768, 768)
            .title("Blask Emulator")
            .build();

        let mut img = Image::gen_image_color(32, 32, Color::BLACK);
        img.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8);

        let tex: Texture2D;

        unsafe {
            tex = Texture2D::from_raw(LoadTextureFromImage(img.to_raw()));
        }

        Self {
            handle: rl,
            thread,
            video: tex,
        }
    }

    /// Check if the Window should be closed.
    pub fn window_should_close(&self) -> bool {
        self.handle.window_should_close()
    }

    /// Set the video buffer with the given buffer.
    pub fn update_video_buffer(&mut self, buffer: &[u16; 32 * 32]) {
        let mut video = self.handle.begin_drawing(&self.thread);

        // Compute required scaling
        let scale = (video.get_screen_width() / 32)
            .min(video.get_screen_height() / 32) as f32;

        let mut pixels: [u8; 32 * 32 * 3] = [0; 32 * 32 * 3];

        for y in 0..32 {
            for x in 0..32 {
                let mut b: u8 = (buffer[x + y * 32] & ((1 << 4) - 1)) as u8;
                let mut g: u8 = ((buffer[x + y * 32] & ((1 << 8) - 1)) >> 4) as u8;
                let mut r: u8 = ((buffer[x + y * 32] & ((1 << 12) - 1)) >> 8) as u8;

                r <<= 4;
                g <<= 4;
                b <<= 4;

                pixels[(x * 3) + (y * 32 * 3)] = r;
                pixels[(x * 3) + 1 + (y * 32 * 3)] = g;
                pixels[(x * 3) + 2 + (y * 32 * 3)] = b;
            }
        }

        self.video.update_texture(&pixels);

        // Draw Texture2D to window, properly scaled
        video.draw_texture_pro(&self.video, 
                               rrect(0.0, 0.0, self.video.width, -self.video.height),
                               rrect((video.get_screen_width() as f32 - (32.0 * scale)) * 0.5,
                                       (video.get_screen_height() as f32 - (32.0 * scale)) * 0.5,
                                       32.0 * scale, 32.0 * scale),
                               rvec2(0, 0),
                               0.0,
                               Color::WHITE);
        // video.draw_texture(&self.video, 0, 0, Color::WHITE);
    }
}
