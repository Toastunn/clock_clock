use std::f32::consts::PI;

use lerp::Lerp;
use rand::random_range;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

use crate::lib::*;
use crate::*;

//clockhand class
pub struct ClockHand {
  pub angle: f32,
  pub rotation: f32,
  pub speed: f32
}

impl ClockHand {
  pub fn default() -> ClockHand {
    ClockHand { angle: Direction::UP, rotation: Rotation::CLOCKWISE, speed: 1. }
  }

  pub fn new(angle: f32) -> ClockHand {
    if (angle / PI) % 1. == 0. {
      ClockHand { angle: angle, rotation: Rotation::CLOCKWISE, speed: 1. }
    } else {
      ClockHand { angle: angle*PI/180., rotation: Rotation::CLOCKWISE, speed: 1. }
    }
  }

  /*
  * Unused functions
  pub fn set_dir(&mut self, direction: f32) {
    self.angle =  direction;
  }

  pub fn get(&self) -> (&f32, &f32) {
    (&self.angle, &self.rotation)
  }
  */

  pub fn render(&self, canvas: &WindowCanvas, center: &Pos) {
    let (dx, dy) = Pos::pol_to_rec(CLOCKHAND_LENGTH, self.angle); //sus
    let (cx, cy) = center.tuple(); 
    canvas.line(cx, cy, cx + dx, cy - dy, HAND_COLOR).unwrap();
  }
}


//subclock class
pub struct SubClock {
  pub center: Pos,
  pub hand1: ClockHand,
  pub hand2: ClockHand
}

impl SubClock {
  pub fn new(center: Pos) -> SubClock {
    SubClock { center: center, hand1: ClockHand::default(), hand2: ClockHand::new(Direction::UP) }
  }

  pub fn set_rotations(&mut self, rot1: f32, rot2: f32) {
    self.hand1.rotation = rot1;
    self.hand2.rotation = rot2;
  }

  pub fn set_speeds(&mut self, s1: f32, s2: f32) {
    self.hand1.speed = s1;
    self.hand2.speed = s2;
  }

  pub fn tick(&mut self, canvas: &WindowCanvas) {
    self.hand1.angle = Angle::check(self.hand1.angle + self.hand1.rotation as f32 * self.hand1.speed * SPEED_FACTOR);
    self.hand2.angle = Angle::check(self.hand2.angle + self.hand2.rotation as f32 * self.hand2.speed * SPEED_FACTOR);
    self.hand1.render(canvas, &self.center);
    self.hand2.render(canvas, &self.center);
  }

  fn set_angles(&mut self, canvas: &WindowCanvas, mut angles: (f32, f32), cycle: u32) {
    if cycle == SCREENSAVER_TIME * FRAMERATE + 1 { 
      let d1 = angles.0 - self.hand1.angle;
      let d2 = angles.1 - self.hand2.angle;

      if d1 > PI || d1 < -PI {
        if self.hand1.angle > angles.0 {
          self.hand1.angle -= 2. * PI;
        } else {
          angles.0 -= 2. * PI
        }
      }
      if d2 > PI || d2 < -PI {
        if self.hand2.angle > angles.1 {
          self.hand2.angle -= 2. * PI;
        } else {
          angles.1 -= 2. * PI
        }
      }
    }

    if cycle <= LERP_TIME * FRAMERATE {
      self.hand1.angle = (angles.0).lerp(self.hand1.angle, 0.95); 
      self.hand2.angle = (angles.1).lerp(self.hand2.angle, 0.95); 
    }

    self.hand1.render(canvas, &self.center);
    self.hand2.render(canvas, &self.center);
  }

}

//digit class
#[derive(Default)]
pub struct Digit {
  pub sub_clocks: Vec<SubClock>
}

impl Digit {
  pub fn new(sub_clocks: Vec<SubClock>) -> Digit{
    Digit { sub_clocks: sub_clocks }
  }

  pub fn set_digit(&mut self, canvas: &WindowCanvas, digit: [(f32, f32); 6], cycle: u32) {
    for index in 0..6 {
      let clock = &mut self.sub_clocks[index];
      let angles = digit[index];

      clock.set_angles(canvas, angles, cycle);
    }
  }

  pub fn tick_foward(&mut self, canvas: &WindowCanvas) {
    for clock in &mut self.sub_clocks {
      clock.tick(canvas);
    }
  }

  pub fn randomize_rotation(&mut self) {
    for clock in &mut self.sub_clocks {
      let rot1 = if rand::random_bool(0.5) {Rotation::CLOCKWISE} else {Rotation::COUNTER_CLOCKWISE};
      let rot2 = if rand::random_bool(0.5) {Rotation::CLOCKWISE} else {Rotation::COUNTER_CLOCKWISE};
      clock.set_rotations(rot1, rot2);
    }
  }

  pub fn randomize_spped(&mut self) {
    for clock in &mut self.sub_clocks {
      let s1 = random_range(0.5..3.);
      let s2 = random_range(0.5..3.);
      clock.set_speeds(s1, s2);
    }
  }
}