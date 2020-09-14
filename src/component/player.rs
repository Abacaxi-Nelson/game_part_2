use amethyst::{
  ecs::prelude::{Component, DenseVecStorage},
  prelude::*,
  renderer::SpriteRender,
};

use crate::sprite::storage::SpriteSheetStorage;
use super::def::Side;
use super::def::Hand;

use rand::Rng;


#[derive(Debug)]
pub struct Player {
  pub side:   Side,
  pub width:  f32,
  pub height: f32,
  pub hand: Hand,
}

impl Player {
  pub fn new(side: Side) -> Player {
    let hand =   match side {
      Side::User     => Hand::PAPER,
      Side::Opponent => Hand::UNKNOWN,
    }; 

    Player {
        side,
        width:  super::PLAYER_WIDTH,
        height: super::PLAYER_HEIGHT,
        hand
    }
  }

  pub fn change_hand(&mut self, mov: i32) {
    let array_hand = [Hand::PAPER, Hand::ROCK, Hand::SCISSORS];
    let current_hand = self.hand as i32;
    let calc:i32 = current_hand + mov;

    if calc == -1 {
      self.hand = array_hand[2];
    }else if calc == 3 {
      self.hand = array_hand[0];
    }else {
      self.hand = array_hand[calc as usize];
    }
  }

  pub fn change_hand_randomly(&mut self) {
    let array_hand = [Hand::PAPER, Hand::ROCK, Hand::SCISSORS];
    let mut rng = rand::thread_rng();
    self.hand = array_hand[rng.gen_range(0, 3)];
  } 

  pub fn get_sprite_render(&self, world: &mut World) -> SpriteRender {
    let sprite_sheet = {
        let sheets = world.read_resource::<SpriteSheetStorage>();
        let sprite_sheet =  match self.side {
          super::def::Side::User     => sheets.paper.clone(),
          super::def::Side::Opponent => sheets.unknown.clone(),
        }; 
        sprite_sheet
    };

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    sprite_render
  }
}

impl Component for Player {
  type Storage = DenseVecStorage<Self>;
}