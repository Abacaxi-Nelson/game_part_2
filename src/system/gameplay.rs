use amethyst::derive::SystemDesc;
use amethyst::ecs::{WriteExpect, Entities, Join, ReadExpect, Write, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender};
use amethyst::ui::UiText;


use crate::component::player::Player;
use crate::sprite::storage::SpriteSheetStorage;
use crate::component::def::Side;
use crate::component::def::Hand;
use crate::state::def::Game;
use crate::state::def::UserAction;
use crate::component::score::ScoreBoard;
use crate::component::score::ScoreText;
use crate::state::def::WinnerMessage;


#[derive(SystemDesc)]
pub struct GamePlaySystem;

impl<'s> System<'s> for GamePlaySystem {
  type SystemData = (
      WriteStorage<'s, Player>,
      Read<'s, InputHandler<StringBindings>>,
      ReadExpect<'s, SpriteSheetStorage>,
      Write<'s, Game>,
      WriteStorage<'s, SpriteRender>,
      Entities<'s>,  
      WriteStorage<'s, UiText>,
      Write<'s, ScoreBoard>,
      WriteExpect<'s, ScoreText>,
      ReadExpect<'s, WinnerMessage>,
  );


  fn run(&mut self, (mut players, input, sheets, mut game, mut sprites, entities, mut ui_text, mut scores, score_text, winner_message): Self::SystemData) {
      for (player, entity) in (&mut players, &entities).join() {
        let change_hand_up   = input.action_is_down("up").unwrap_or(false)       && game.user_action == UserAction::ChoosingHand; 
        let change_hand_down = input.action_is_down("down").unwrap_or(false)     && game.user_action == UserAction::ChoosingHand;
        let validate_hand    = input.action_is_down("validate").unwrap_or(false) && game.user_action == UserAction::ChoosingHand; 
        let next_game        = input.action_is_down("next").unwrap_or(false)     && game.user_action == UserAction::ShowWinner; 

        //println!("game.user_action: {:?}", game.user_action);
        //println!("change_hand_up: {}", change_hand_up);
        
        //if validate_hand  {
         // println!("VALIDATE HAND: //////////////////////////////////////////////");
         // println!("VALIDATE HAND: {}", input.action_is_down("validate").unwrap_or(false));
        //  println!("VALIDATE HAND: {}", game.user_action == UserAction::ChoosingHand);
        //  println!("VALIDATE HAND: {}", validate_hand);
        //}
        //println!("VALIDATE HAND: {}", game.user_action == UserAction::ChoosingHand);
        //println!("VALIDATE HAND: {}", validate_hand);
        //println!("VALIDATE HAND: //////////////////////////////////////////////");

        // remove winner text

        match player.side {
          Side::User     => {
            // here we are sure, we loop from a user, so here we have all his actions
            // change tile of the player
            if change_hand_up || change_hand_down {
                handle_change_hand_user(change_hand_up, player);
                let sprite_sheet = match player.hand {
                  Hand::PAPER    =>  sheets.paper.clone(),
                  Hand::ROCK     => sheets.rock.clone(),
                  Hand::SCISSORS => sheets.scissors.clone(),
                  Hand::UNKNOWN => sheets.unknown.clone(), 
                }; 

                sprites
                .insert(
                  entity,
                    SpriteRender {
                        sprite_sheet: sprite_sheet,
                        sprite_number: 0,
                    },
                )
                .unwrap();
            }

            // player validate his hand
            if validate_hand  {
                game.player_hand = player.hand.clone();
                game.user_action = UserAction::ValidateHand;
            }

            // player want to replay
            if next_game  {
              if let Some(winner_text) = ui_text.get_mut(winner_message.winner_message) {
                winner_text.text = "".to_string();
              }

              game.user_action = UserAction::ChoosingHand;
            }
          }    
          Side::Opponent => {
            // here we reflect change from player for tghe opponent
            match game.user_action {
              UserAction::ShowWinner    => {

              }
              UserAction::ValidateHand    => {
                // Random, choose a hand, calculate the winner
                player.change_hand_randomly();
                game.opponent_hand = player.hand.clone();
                let is_user_winner = game.is_user_winner();

                let sprite_sheet = match player.hand {
                  Hand::PAPER    =>  sheets.paper.clone(),
                  Hand::ROCK     => sheets.rock.clone(),
                  Hand::SCISSORS => sheets.scissors.clone(),
                  Hand::UNKNOWN => sheets.unknown.clone(), 
                }; 

                println!("ON SE MET A JOUR: {:?}", player.hand);

                //Replace sprite
                sprites
                .insert(
                  entity,
                    SpriteRender {
                        sprite_sheet: sprite_sheet,
                        sprite_number: 0,
                    },
                )
                .unwrap();
                
                //println!("VALIDATE HAND: {}", validate_hand);
                println!("ON CHANGE LE SCORE");

                // dirty send an enum type= User, Opponent instead
                if is_user_winner == 1 {
                  println!("1.1 {}", scores.user_score);
                  scores.user_score = (scores.user_score + 1).min(999);
                  println!("1.2 {}", scores.user_score);
                  if let Some(text) = ui_text.get_mut(score_text.user_score) {
                    println!("{}", scores.user_score.to_string());
                    text.text = scores.user_score.to_string();
                  }
                  // SET message winner
                  if let Some(winner_text) = ui_text.get_mut(winner_message.winner_message) {
                    winner_text.text = "YOU WIN".to_string();
                  }
                }else if is_user_winner == 2 {
                  scores.opponent_score = (scores.opponent_score + 1).min(999);
                  println!("2 {}", scores.opponent_score);
                  if let Some(text) = ui_text.get_mut(score_text.opponent_score) {
                    println!("{}", scores.opponent_score.to_string());
                    text.text = scores.opponent_score.to_string();
                  }
                  if let Some(winner_text) = ui_text.get_mut(winner_message.winner_message) {
                    winner_text.text = "YOU LOOSE".to_string();
                  }
                }else{
                  if let Some(winner_text) = ui_text.get_mut(winner_message.winner_message) {
                    winner_text.text = "NO WINNER".to_string();
                  }
                }
              
                game.user_action = UserAction::ShowWinner;
              }
              // reset his hand
              UserAction::ChoosingHand => {
                match player.hand {
                  Hand::UNKNOWN => (),
                  _ =>  {
                    player.hand = Hand::UNKNOWN;
                    let sprite_sheet = sheets.unknown.clone();
                    sprites
                    .insert(
                      entity,
                        SpriteRender {
                            sprite_sheet: sprite_sheet,
                            sprite_number: 0,
                        },
                    )
                    .unwrap();
                  }  
                }  
              }
            }; 
          } 
        }
      }
  }
}

fn handle_change_hand_user(change_hand_up: bool, player: &mut Player){
  let calc = if change_hand_up {
    1
  } else {
    -1
  };
  
  player.change_hand(calc);
}
