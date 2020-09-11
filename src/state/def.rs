use crate::component::player::Player;

pub struct Game{
  pub current_state: CurrentState,
  pub user_action: UserAction
}

impl Default for Game {
  fn default() -> Self {
    Game {
      current_state: CurrentState::default(),
      user_action: UserAction::ChoosingHand
    }
  }
}

pub enum CurrentState {
  GamePlay,
}

impl Default for CurrentState{
  fn default() -> Self {
    CurrentState::GamePlay
  }
}
pub enum UserAction {
  ChoosingHand,
  ValidateHand,
}
