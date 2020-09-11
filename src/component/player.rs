use amethyst::{
  ecs::prelude::{Component, DenseVecStorage},
  prelude::*,
  renderer::{SpriteRender},
};
use crate::sprite::storage::SpriteSheetStorage;

pub struct Player {
  pub side:   super::def::Side,
  pub width:  f32,
  pub height: f32,
}

impl Player {
  pub fn new(side: super::def::Side) -> Player {
    Player {
        side,
        width:  super::PLAYER_WIDTH,
        height: super::PLAYER_HEIGHT,
    }
  }

  pub fn get_sprite_render(&self, world: &mut World) -> SpriteRender {
    let sprite_sheet = {
        let sheets = world.read_resource::<SpriteSheetStorage>();
        let sprite_sheet =  match self.side {
          super::def::Side::User     => sheets.rock.clone(),
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