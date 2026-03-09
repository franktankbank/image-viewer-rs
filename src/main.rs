mod root;

use sdl2::{self, event::Event, video::{GLProfile, Window}};
use imgui_sdl2_support::SdlPlatform;
use imgui::{Context, TextureId};
use imgui_glow_renderer::{
    AutoRenderer, glow::{self, HasContext, NativeTexture}
};
use rfd::FileDialog;
use std::fs::File;

use root::{error, img::{ImageData, jxl, xpm, qoi}};

struct ImageDims {
    draw_w: f32,
    draw_h: f32,
    x: f32,
    y: f32
}

struct RenderConfig {
    image_data: ImageData,
    texture: Texture
}

struct Texture {
    id: TextureId,
    texture: NativeTexture
}

enum ImageFormat {
    Jxl,
    Xpm,
    Qoi
}

impl RenderConfig {
    fn get_dims(&self, display: [f32; 2]) -> ImageDims {
        let img_w = self.image_data.width as f32;
        let img_h = self.image_data.height as f32;
        let scale = f32::min(display[0] / img_w, display[1] / img_h);
        let draw_w = img_w * scale;
        let draw_h = img_h * scale;

        let x = (display[0] - draw_w) * 0.5;
        let y = (display[1] - draw_h) * 0.5;

        ImageDims {
            draw_w: draw_w,
            draw_h: draw_h,
            x: x,
            y: y
        }
    }
}

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn identify_image(image_path: &str) -> Result<ImageFormat, error::ImageIdentityError> {
    let db = magic_db::global()?;

    let mut file = File::open(image_path)?;
    let magic = db.first_magic(&mut file, None)?;
    assert!(!magic.is_default());

    match magic.mime_type() {
        "image/jxl" => Ok(ImageFormat::Jxl),
        "image/x-xpixmap" => Ok(ImageFormat::Xpm),
        "image/x-qoi" => Ok(ImageFormat::Qoi),
        _ => Err(error::ImageIdentityError::UnsupportedImage(error::UnsupportedImageError))
    }
}

fn render_image(image_path: &str, gl: &glow::Context, window: &mut Window) -> Result<RenderConfig, error::ImageViewerError> {
    let image_identity = identify_image(image_path)?;

    let data: ImageData = match image_identity {
        ImageFormat::Jxl => {
            window.set_title("Image Viewer (JPEG XL)")?;
            jxl::decode_jxl(image_path)?
        },
        ImageFormat::Xpm => {
            window.set_title("Image Viewer (X Pixmap)")?;
            xpm::decode_xpm(image_path)?
        },
        ImageFormat::Qoi => {
            window.set_title("Image Viewer (Quite OK)")?;
            qoi::decode_qoi(image_path)?
        }
    };

    let texture = unsafe {
        let tex = gl.create_texture().unwrap();
        let rgba = data.image.to_rgba8();
        let pixels = rgba.as_raw();

        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::SRGB8_ALPHA8 as i32,
            data.width as i32,
            data.height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(pixels),
        );

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.generate_mipmap(glow::TEXTURE_2D);

        tex
    };

    let texture_id = imgui::TextureId::from(texture.0.get() as usize);

    let texture_config = Texture {
        id: texture_id,
        texture: texture
    };

    let render_config = RenderConfig {
        image_data: data,
        texture: texture_config
    };

    Ok(render_config)
}

fn open_image(renderer: &mut AutoRenderer, render_config: &mut Option<RenderConfig>, window: &mut Window) -> Result<(), error::ImageViewerError>{
    if let Some(path) = FileDialog::new()
    .add_filter("JPEG XL", &["jxl"])
    .add_filter("X Pixmap", &["xpm"])
    .add_filter("Quite OK", &["qoi"])
    .set_title("Image Viewer").pick_file() {
        if let Some(old) = &render_config {
            unsafe {
                renderer.gl_context().delete_texture(old.texture.texture);
            }
        }
        *render_config = Some(render_image(
            path.to_str().unwrap(),
            renderer.gl_context(),
            window
        )?);

        Ok(())
    } else {
        Err(error::ImageViewerError::OpenImage(error::OpenImageError))
    }
}

fn main() -> Result<(), error::ImageViewerError> {
    image_extras::register();
    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let mut window = video_subsystem
        .window("Image Viewer", 800, 600)
        .position_centered()
        .allow_highdpi()
        .opengl()
        .build()?;

    let gl_context = window.gl_create_context()?;
    window.gl_make_current(&gl_context)?;

    window.subsystem().gl_set_swap_interval(1)?;

    let gl = glow_context(&window);

    let mut imgui = Context::create();

    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui)?;

    let mut event_pump = sdl2_context.event_pump()?;

    let mut render_config: Option<RenderConfig> = None;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::W),
                    keymod,
                    ..
                } if keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) => {
                    break 'mainloop;
                }

                Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::O),
                    keymod,
                    ..
                } if keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) => {
                    open_image(&mut renderer, &mut render_config, &mut window)?
                }

                Event::Quit { .. } => {
                    break 'mainloop
                }
                _ => {}
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);
        {
            let ui = imgui.new_frame();
            if let Some(_menu_bar) = ui.begin_main_menu_bar() {
                if let Some(_menu) = ui.begin_menu("File") {
                    let open = ui.menu_item_config("Open").shortcut("Ctrl+O").build();
                    if open {
                        open_image(&mut renderer, &mut render_config, &mut window)?;
                    }
                    let close = ui.menu_item_config("Close").shortcut("Ctrl+W").build();
                    if close {
                        break 'mainloop
                    }
                    _menu.end();
                }
            }
            if let Some(config) = &render_config {
                let draw_list = ui.get_background_draw_list();
                let display = ui.io().display_size;
                let dims = config.get_dims(display);

                draw_list
                    .add_image(
                        config.texture.id,
                        [dims.x, dims.y],
                        [dims.x + dims.draw_w, dims.y + dims.draw_h]
                    )
                    .build();
            }
        }
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data)?;

        window.gl_swap_window();
    }

    Ok(())
}
