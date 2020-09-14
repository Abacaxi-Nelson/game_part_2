use amethyst::{
  ecs::prelude::{Entity},
  prelude::*,
  assets::{Loader},
  ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct ScoreBoard {
    pub user_score: i32,
    pub opponent_score: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub user_score: Entity,
    pub opponent_score: Entity,
}

impl ScoreText {
    pub fn new(world: &mut World, user_transform: UiTransform, opponent_transform:UiTransform)  -> ScoreText {
        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );
        
        let user_score = world
            .create_entity()
            .with(user_transform)
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [0., 0., 0., 1.],
                150.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .build();

        let opponent_score = world
            .create_entity()
            .with(opponent_transform)
            .with(UiText::new(
                font,
                "0".to_string(),
                [0., 0., 0., 1.],
                150.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .build();

        ScoreText{
            user_score,
            opponent_score,
        }
    }
}


