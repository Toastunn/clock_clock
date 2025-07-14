#[allow(non_snake_case)]
#[path = "clock/UIElements.rs"]
mod UIElements;
#[path = "clock/lib.rs"]
mod lib;

use std::time::Duration;

use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::ttf;

use crate::UIElements::*;
use crate::lib::*;

const WINDOW_WIDTH: u32                 = 1940;
const WINDOW_HEIGHT: u32                = 1040;
const FRAMERATE: u32                    = 60;

const CLOCKHAND_LENGTH: f32             = 100.;
const OFFSET: i16                       = (CLOCKHAND_LENGTH * 2.) as i16;
const PADDING: i16                      = (CLOCKHAND_LENGTH / 2. + CLOCKHAND_LENGTH) as i16;
const DIGIT_GAP: i16                    = OFFSET * 2 + (CLOCKHAND_LENGTH / 4.) as i16;

const HAND_COLOR: Color                 = Color::RED;
const BG_COLOR: Color                   = Color::BLACK;

const SPEED_FACTOR: f32                 = 0.01;

const SCREENSAVER_TIME: u32             = 10;
const LERP_TIME: u32                    = 20;
const CLOCK_DISPLAY_TIME: u32           = 20; // CLOCK_DISPLAY_TIME lasts for SCREENSAVER_TIME - CLOCK_DISPLAY_TIME

const FIRST_DIGIT: usize = 0;
const SECOND_DIGIT: usize = 1;
const THIRD_DIGIT: usize = 2;
const FOURTH_DIGIT: usize = 3;

#[allow(unused_parens)]
fn main() -> Result<(), String> {

  let sdl_context = sdl2::init()?;
  let _ttf_context = ttf::init().expect("Fail to load font");
  let video_subsystem = sdl_context.video()?;

  let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

  let window = video_subsystem.window("Clock Clock", WINDOW_WIDTH, WINDOW_HEIGHT)
    .position_centered()
    .build()
    .expect("Initializing video subsystem failed");

  let mut canvas = window.into_canvas().build()
    .expect("Creating canvas failed");

  let mut clock_display = [
    Digit::new(vec![
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 0), PADDING + OFFSET * 0)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 0), PADDING + OFFSET * 0)),
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 0), PADDING + OFFSET * 1)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 0), PADDING + OFFSET * 1)),
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 0), PADDING + OFFSET * 2)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 0), PADDING + OFFSET * 2)),
    ]),
    Digit::new(vec![
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 1), PADDING + OFFSET * 0)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 1), PADDING + OFFSET * 0)),
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 1), PADDING + OFFSET * 1)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 1), PADDING + OFFSET * 1)),
      SubClock::new(Pos::new((PADDING * 1 + DIGIT_GAP * 1), PADDING + OFFSET * 2)), SubClock::new(Pos::new((PADDING * 1 + OFFSET + DIGIT_GAP * 1), PADDING + OFFSET * 2)),
    ]),
    Digit::new(vec![
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 2), PADDING + OFFSET * 0)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 2), PADDING + OFFSET * 0)),
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 2), PADDING + OFFSET * 1)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 2), PADDING + OFFSET * 1)),
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 2), PADDING + OFFSET * 2)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 2), PADDING + OFFSET * 2)),
    ]),
    Digit::new(vec![
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 3), PADDING + OFFSET * 0)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 3), PADDING + OFFSET * 0)),
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 3), PADDING + OFFSET * 1)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 3), PADDING + OFFSET * 1)),
      SubClock::new(Pos::new((PADDING * 2 + DIGIT_GAP * 3), PADDING + OFFSET * 2)), SubClock::new(Pos::new((PADDING * 2 + OFFSET + DIGIT_GAP * 3), PADDING + OFFSET * 2)),
    ]),
  ];

  let mut time: Time24h = Time24h::get_current();

  let mut cycle = 0;
  let mut event_pump = sdl_context.event_pump()?;

  'running: loop {

    // Handle Events
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} => {
          break 'running;
        },
        _ => {}
      }
    }
    //Updates (if necessary)
    cycle += 1;

    //Queue rendering
    canvas.set_draw_color(BG_COLOR);
    canvas.clear();

    if cycle <= SCREENSAVER_TIME * FRAMERATE {
      for digit in &mut clock_display {
          digit.tick_foward(&canvas);
      };
      time = Time24h::get_current();
    }
    else if cycle <= CLOCK_DISPLAY_TIME * FRAMERATE {

      let hour_digits   = DigitMap::time_to_digit(&time.hour);
      let minute_digits = DigitMap::time_to_digit(&time.minute);

      clock_display[FIRST_DIGIT] .set_digit(&canvas, hour_digits  [FIRST_DIGIT],  cycle);
      clock_display[SECOND_DIGIT].set_digit(&canvas, hour_digits  [SECOND_DIGIT], cycle);
      clock_display[THIRD_DIGIT] .set_digit(&canvas, minute_digits[FIRST_DIGIT],  cycle);
      clock_display[FOURTH_DIGIT].set_digit(&canvas, minute_digits[SECOND_DIGIT], cycle);
    }
    else {
      cycle = 0;
      for digit in &mut clock_display {
          digit.randomize_rotation();
          digit.randomize_spped();
      };
      
    }

    //last thing: present the canvas
    canvas.present();

    // Time management
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMERATE));
  }

  Ok(())
}