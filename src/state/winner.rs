use amethyst::{
  prelude::*,
};

use super::def::Game;
use super::def::UserAction;
use super::def::CurrentState;
use super::gameplay::GameplayState;

pub struct WinnerState;

impl SimpleState for WinnerState {
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {  
      let game = data.world.write_resource::<Game>();
      match game.user_action {
          UserAction::ChoosingHand => return Trans::Push(Box::new(GameplayState)),
          _ => Trans::None,
      }
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().current_state = CurrentState::Winner;
    }
}