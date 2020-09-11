use amethyst::{
  ecs::prelude::{Entity},
  prelude::*,
  assets::{Loader},
  ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},

};

pub struct Rules {
    pub rule_message: Entity,
}

impl Rules {
    pub fn new(world: &mut World, message_transform: UiTransform)  -> Rules {
        let message = "Press Top or Down to choose your hand. Validate with space.";

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );
        
        let rule_message = world
            .create_entity()
            .with(message_transform)
            .with(UiText::new(
                font,
                message.to_string(),
                [0., 0., 0., 1.],
                30.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .build();

            Rules{
                rule_message
            }
    }
}


