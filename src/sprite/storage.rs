use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  prelude::*,
  renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct SpriteSheetStorage {
  pub paper: Handle<SpriteSheet>,
  pub rock: Handle<SpriteSheet>,
  pub scissors: Handle<SpriteSheet>,
  pub unknown: Handle<SpriteSheet>,
}

impl SpriteSheetStorage {
  pub fn new(world: &mut World) -> Self {
      SpriteSheetStorage {
          paper:    load_sprite_sheet(world, "paper").clone(),
          rock:     load_sprite_sheet(world, "rock").clone(),
          scissors: load_sprite_sheet(world, "scissors").clone(),
          unknown:  load_sprite_sheet(world, "unknown").clone(),
      }
  }
}

pub fn load_sprite_sheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
  let texture_handle = {
      let loader = world.read_resource::<Loader>();
      let texture_storage = world.read_resource::<AssetStorage<Texture>>();
      loader.load(
          format!("textures/{}/spritesheet.png", name),
          ImageFormat::default(),
          (),
          &texture_storage,
      )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
      format!("textures/{}/spritesheet.ron", name),
      SpriteSheetFormat(texture_handle),
      (),
      &sprite_sheet_store,
  )
}