#[derive(Debug)]
pub mod vegetables;

pub mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
  pub struct Rectangle {
    width: u32,
    height: u32,
  }

  pub impl Rectangle {
      pub fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
}