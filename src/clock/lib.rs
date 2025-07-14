use std::{f32::consts::PI};

use chrono::*;

pub struct  Direction {}

impl Direction {
  pub const RIGHT: f32 = 0.;
  pub const UP_RIGHT: f32 = PI/4.;
  pub const UP: f32 = 2.*PI/4.;
  pub const UP_LEFT: f32 = 3.*PI/4.;
  pub const LEFT: f32 = 4.*PI/4.;
  pub const DOWN_LEFT: f32 = 5.*PI/4.;
  pub const DOWN: f32 = 6.*PI/4.;
  pub const DOWN_RIGHT: f32 = 7.*PI/4.;
}

pub struct Rotation {}

impl Rotation {
  pub const CLOCKWISE: f32 = -1.;
  pub const COUNTER_CLOCKWISE: f32 = 1.;
}

pub struct Angle {}

impl Angle {
  pub fn check(angle: f32) -> f32 {
    let mut buf = angle;
    while buf < 0. {
      buf += 2. * PI;
    }
    while buf > 2. * PI {
      buf -= 2. * PI;
    }
    buf % (2. * PI)
  }
}

pub struct Pos {
  pub x: i16,
  pub y: i16
}

impl Pos {
  pub fn new(x: i16, y: i16) -> Pos {
    Pos { x: x, y: y}
  }

  pub fn pol_to_rec(r: f32, angle: f32) -> (i16, i16) {
    let x = r*f32::cos(angle);
    let y = r*f32::sin(angle);
    (x as i16, y as i16) //hmmm
  }

  /*
  * This function is unused for now
  pub fn rec_to_pol(xi: i16, yi: i16) -> f32 {
    let x = xi as f32;
    let y = yi as f32;

    let mut angle = 0.;
    println!();
    if x == 0. {
      if y < 0. {angle = PI/2.;}
      if y > 0. {angle = 3.*PI;}
    } else
    if y == 0. {
      if x < 0. {angle = PI;}
      if x > 0. {angle = 00.;}
    } else
    if x > 0. {
      // -pi/2 to pi/2
      angle = f32::atan(y/x);
      angle /= 2.; 
      println!("+, +/-")
    } else
    if x < 0. {
      if y > 0. {
      // pi/2 to pi
        angle = PI - f32::atan(y.abs()/x.abs());
        println!("-, +")
      }
      if y < 0. {
      // -pi to -pi/2
        angle = (f32::atan(y.abs()/x.abs())) - PI;
        println!("{angle}");
        println!("-, -")
      }
    }

    println!("{}", angle);

    angle
  }
   */

  pub fn tuple(&self) -> (i16, i16) {
    (self.x, self.y)
  }
}

#[derive(Debug)]
pub struct Time24h {
  pub hour: String,
  pub minute: String
}

impl Time24h {
  pub fn new(hour: String, minute: String) -> Time24h {
    Time24h { hour: hour, minute: minute }
  }
  pub fn get_current() -> Time24h {
    let time = Local::now().time().format("%H:%M");
    let mut time_vec = time.to_string().split(":").map(|s| s.to_string()).collect::<Vec<_>>();
    time_vec.reverse();
    Time24h::new(time_vec.pop().unwrap(), time_vec.pop().unwrap())
  }
}

pub struct DigitMap {}

impl DigitMap {
  pub const ONE: [(f32, f32); 6] = [
    (Direction::UP_RIGHT, Direction::UP_RIGHT), (Direction::DOWN_LEFT, Direction::DOWN),
    (Direction::UP_RIGHT, Direction::UP_RIGHT), (Direction::UP, Direction::DOWN),
    (Direction::UP_RIGHT, Direction::UP_RIGHT), (Direction::UP, Direction::UP),
  ];
  pub const TWO: [(f32, f32); 6] = [
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::UP),
    (Direction::UP, Direction::RIGHT), (Direction::LEFT, Direction::LEFT),
  ];
  pub const THREE: [(f32, f32); 6] = [
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::RIGHT, Direction::RIGHT), (Direction::UP, Direction::DOWN),
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];
  pub const FOUR: [(f32, f32); 6] = [
    (Direction::UP_LEFT, Direction::UP_LEFT), (Direction::DOWN_LEFT, Direction::DOWN),
    (Direction::UP_RIGHT, Direction::RIGHT), (Direction::DOWN, Direction::RIGHT),
    (Direction::DOWN_LEFT, Direction::DOWN_LEFT), (Direction::UP, Direction::UP),
  ];
  pub const FIVE: [(f32, f32); 6] = [
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::LEFT),
    (Direction::UP, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];
  pub const SIX: [(f32, f32); 6] = [
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::LEFT),
    (Direction::UP, Direction::DOWN), (Direction::LEFT, Direction::DOWN),
    (Direction::UP, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];
  pub const SEVEN: [(f32, f32); 6] = [
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::DOWN_LEFT, Direction::DOWN_LEFT), (Direction::UP, Direction::DOWN),
    (Direction::DOWN_LEFT, Direction::DOWN_LEFT), (Direction::UP, Direction::UP),
  ];
  pub const EIGHT: [(f32, f32); 6] = [
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::UP, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];
  pub const NINE: [(f32, f32); 6] = [
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::UP, Direction::RIGHT), (Direction::DOWN, Direction::UP),
    (Direction::RIGHT, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];
  pub const ZERO: [(f32, f32); 6] = [
    (Direction::DOWN, Direction::RIGHT), (Direction::LEFT, Direction::DOWN),
    (Direction::DOWN, Direction::UP), (Direction::DOWN, Direction::UP),
    (Direction::UP, Direction::RIGHT), (Direction::LEFT, Direction::UP),
  ];

  pub fn time_to_digit(time: &String) -> Vec<[(f32, f32); 6]> {
    time.chars().map(|t| Self::map_to_map(t).expect("Time should be provided")).collect()
  }

  fn map_to_map(t: char) -> Result<[(f32, f32); 6], String> {
    match t {
      '0' => Ok(DigitMap::ZERO),
      '1' => Ok(DigitMap::ONE),
      '2' => Ok(DigitMap::TWO),
      '3' => Ok(DigitMap::THREE),
      '4' => Ok(DigitMap::FOUR),
      '5' => Ok(DigitMap::FIVE),
      '6' => Ok(DigitMap::SIX),
      '7' => Ok(DigitMap::SEVEN),
      '8' => Ok(DigitMap::EIGHT),
      '9' => Ok(DigitMap::NINE),
      _ => Err("Not a valid time!".to_string())
    }
  }
}