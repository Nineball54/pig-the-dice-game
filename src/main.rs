/* Pig the dice game
 * The game of Pig is a multiplayer game played with a single 6-sided die. The object of the game
 * is to reach 100 points or more. Play is taken in turns. On each person's turn that person has
 * the option of either:
 *   1. Rolling the dice: where a roll of two to six is added to their score for that turn and the
 *      player's turn continues as the player is given the same choice again; or a roll of 1 loses
 *      the player's total points for that turn and their turn finishes with play passing to the
 *      next player.
 *
 *   2. Holding: The player's score for that round is added to their total and becomes safe from
 *      the effects of throwing a 1. The player's turn finishes with play passing to the next
 *      player.
 *
 *   SEE:
 *     https://rosettacode.org/wiki/Pig_the_dice_game
 */

/*
 *   TASK:
 *     Create a program to track score and simulate dice throws in a two-player game.
*/

use rand::prelude::*;

fn main() {
    println!("Beginning game of Pig...");

    Stage::new(
        Player::new(String::from("PLAYER (1) ONE")),
        Player::new(String::from("PLAYER (2) TWO")),
    )
    .perform();

    println!("Thanks for playing!");
}

type DiceRoll = u32;
type Score = u32;
type Name = String;

enum Action {
    Roll,
    Hold,
}

#[derive(PartialEq)]
enum TurnStatus {
    Continue,
    End,
}

struct Player {
    name: Name,
    score: Score,
    status: TurnStatus,
}

impl Player {
    fn new(name: Name) -> Player {
        Player {
            name,
            score: 0,
            status: TurnStatus::Continue,
        }
    }

    fn roll() -> DiceRoll {
        // Simple 1d6 dice.
        let sides = rand::distributions::Uniform::new(1, 6);
        rand::thread_rng().sample(sides)
    }

    fn action() -> Action {
        // Closure to determine userinput as action.
        let command = || -> char {
            let mut cmd: String = String::new();
            match std::io::stdin().read_line(&mut cmd) {
                Ok(c) => c.to_string(),
                Err(err) => panic!("Error: {}", err),
            };

            cmd.to_lowercase()
                .chars()
                .next()
                .expect("could not retrieve first letter!")
        };

        match command() {
            'r' => Action::Roll,
            'h' => Action::Hold,
            _ => panic!("not a valid command!"),
        }
    }

    fn turn(&mut self) -> Score {
        let mut score: Score = 0;
        'player: loop {
            println!("# {}'s Turn", self.name);
            println!("######  [R]oll   ######\n######  --OR--   ######\n######  [H]old   ######");
            match Player::action() {
                Action::Roll => match Player::roll() {
                    0 | 7..=u32::MAX => panic!("outside dice bounds!"),
                    die @ 1 => {
                        println!("[DICE] Dice result is: {:3}!", die);
                        println!("[DUMP] Dumping Score! Sorry!");
                        println!("###### ENDING TURN ######");
                        self.status = TurnStatus::End;
                        break 'player 0;
                    }
                    die @ 2..=6 => {
                        println!("[DICE] Dice result is: {:3}!", die);
                        println!("[ROLL] Total    Score: {:3}!", (score + die));
                        println!("[HOLD] Possible Score: {:3}!", (score + die + self.score));
                        self.status = TurnStatus::Continue;
                        score += die
                    }
                },
                Action::Hold => {
                    self.status = TurnStatus::End;
                    break 'player score;
                }
            }
        }
    }

    fn resolve(&mut self) {
        self.score += self.turn()
    }
}

struct Stage {
    players: Vec<Player>,
}

impl Stage {
    fn new(player_a: Player, player_b: Player) -> Stage {
        Stage {
            players: vec![player_a, player_b],
        }
    }

    fn perform(&mut self) {
        'game: loop {
            for player in &mut self.players {
                if player.score <= 100 || player.status == TurnStatus::Continue {
                    println!("\n# {} has {:?} Score", player.name, player.score);
                    player.resolve();
                } else {
                    println!("\n{} wins!", player.name);
                    break 'game;
                }
            }
        }
    }
}
