use amethyst::{
  ecs::prelude::{Component, DenseVecStorage, Entity},
  prelude::*,
  assets::{Loader},
  ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

use crate::component::def::Hand;

pub struct Game{
  pub current_state: CurrentState,
  pub user_action: UserAction,
  pub player_hand: Hand,
  pub opponent_hand: Hand,
  pub last_winner: u32,
}

impl Default for Game {
  fn default() -> Self {
    Game {
      current_state: CurrentState::default(),
      user_action: UserAction::ChoosingHand,
      player_hand: Hand::UNKNOWN,
      opponent_hand: Hand::UNKNOWN,
      last_winner: 0
    }
  }
}

impl Game {
  pub fn is_user_winner(&mut self) -> u32 {
    let result = match self.player_hand {
      Hand::PAPER    =>  {
        match self.opponent_hand {
          Hand::PAPER    =>  0,
          Hand::ROCK     =>  1,
          Hand::SCISSORS => 2,
          Hand::UNKNOWN => 0, 
        }
      },
      Hand::ROCK     => {
        match self.opponent_hand {
          Hand::PAPER    =>  2,
          Hand::ROCK     =>  0,
          Hand::SCISSORS => 1,
          Hand::UNKNOWN => 0, 
        }
      },
      Hand::SCISSORS => {
        match self.opponent_hand {
          Hand::PAPER    =>  1,
          Hand::ROCK     =>  2,
          Hand::SCISSORS => 0,
          Hand::UNKNOWN => 0, 
        }
      },
      Hand::UNKNOWN => 0 // handle an error instead 
    };
    self.last_winner = result;
    return result;
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
  GamePlay,
  Winner
}

impl Default for CurrentState{
  fn default() -> Self {
    CurrentState::GamePlay
  }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction {
  ChoosingHand,
  ValidateHand,
  ShowWinner,
}

#[derive(Debug)]
pub struct WinnerMessage {
  pub winner_message: Entity,
}

impl WinnerMessage {
  pub fn new(world: &mut World, message_transform: UiTransform, message: String)  -> WinnerMessage {
      let font = world.read_resource::<Loader>().load(
          "font/square.ttf",
          TtfFormat,
          (),
          &world.read_resource(),
      );
      
      let winner_message = world
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

      WinnerMessage{
        winner_message
      }
  }
}

impl Component for WinnerMessage {
  type Storage = DenseVecStorage<Self>;
}