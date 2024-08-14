extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;
 // SDL_bool done = SDL_FALSE;

 //    InitVideo();
 //    /* ... */

 //    SDL_StartTextInput();
 //    while (!done) {
 //        SDL_Event event;
 //        if (SDL_PollEvent(&event)) {
 //            switch (event.type) {
 //                case SDL_QUIT:
 //                    /* Quit */
 //                    done = SDL_TRUE;
 //                    break;
 //                case SDL_TEXTINPUT:
 //                    /* Add new text onto the end of our text */
 //                    strcat(text, event.text.text);
 //                    break;
 //                case SDL_TEXTEDITING:
 //                    /*
 //                    Update the composition text.
 //                    Update the cursor position.
 //                    Update the selection length (if any).
 //                    */
 //                    composition = event.edit.text;
 //                    cursor = event.edit.start;
 //                    selection_len = event.edit.length;
 //                    break;
 //            }
 //        }
 //        Redraw();
 //    }

 //    SDL_Quit();


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let input_rect = get_centered_rect(
        500,
        SCREEN_WIDTH - 10,
        SCREEN_WIDTH - 10,
        SCREEN_HEIGHT - 10
    );
    // video_subsystem.text_input().set_rect(input_rect);
    video_subsystem.text_input().start();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut font = ttf_context.load_font("/var/home/bleggett/DroidSansMono.ttf", 24).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let _ = canvas.draw_rect(input_rect);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // let _ = canvas.draw_rect(input_rect);
    // canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut in_text: String = "".into();
    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    in_text.pop();
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    in_text.push('\n');
                },
                Event::TextInput { text, .. } => {
                    println!("TEXT IS: {:?}", text);
                    in_text.push_str(&text);

                },
                Event::TextEditing { text, start, length, .. } => {
                    println!("TEXT IS: {:?}, LENGTH IS: {:?}", text, length);
                },
                _ => {}
            }
        }

        if in_text.len() > 0 {

            // The rest of the game loop goes here...
            // render a surface, and convert it to a texture bound to the canvas
            let texture_creator = canvas.texture_creator();
            let surface = font
                .render(&in_text)
                .blended_wrapped(Color::RGBA(255, 0, 0, 255), 400)
                .map_err(|e| e.to_string()).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string()).unwrap();

            // If the example text is too big for the screen, downscale it (and center irregardless)
            let padding = 64;

            let TextureQuery { width, height, .. } = texture.query();

            let target = rect!(
                input_rect.x(),
                input_rect.y(),
                width,
                height
            );

            canvas.clear();
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            let _ = canvas.draw_rect(input_rect);
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.copy(&texture, None, Some(target));

            canvas.present();

        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, 50)
}
