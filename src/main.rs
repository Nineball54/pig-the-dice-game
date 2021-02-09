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
 *   TASK:
 *     Create a program to track score and simulate dice throws in a two-player game.
 *
 *   SEE:
 *     https://rosettacode.org/wiki/Pig_the_dice_game
 */

use rand::prelude::*;

fn main() {
    println!("Beginning game of Pig...");
    let player1: Player = Player::new("PLAYER ONE (1)");
    let player2: Player = Player::new("PLAYER TWO (2)");

    let mut stage: Stage = Stage::new(player1, player2);
    stage.perform();
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Roll,
    Hold,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum TurnStatus {
    Continue,
    End,
}

type Score = u32;
type Name<'name> = &'name str;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
struct Player<'name> {
    name: Name<'name>,
    score: Score,
    status: TurnStatus,
}

impl<'name> Player<'_> {
    fn new(name: Name) -> Player {
        Player {
            name,
            score: 0,
            status: TurnStatus::Continue,
        }
    }

    fn action() -> Action {
        let command = || -> String {
            let mut cmd: String = String::new();
            match std::io::stdin().read_line(&mut cmd) {
                Ok(c) => c.to_string(),
                Err(err) => panic!("Error: {}", err),
            };

            cmd.trim().parse().expect("could not complete string")
        };

        let fetch = |cmd: String| -> char {
            cmd.as_str()
                .to_lowercase()
                .chars()
                .next()
                .expect("could not retrieve first letter!")
        };

        match fetch(command()) {
            'r' => Action::Roll,
            'h' => Action::Hold,
            _ => panic!("not a valid command!"),
        }
    }

    fn turn(&mut self) -> Score {
        let mut score: Score = 0;
        'player: loop {
            println!("\n# {}'s Turn", self.name);
            println!("######  [R]oll   ######\n######  --OR--   ######\n######  [H]old   ######");
            match Player::action() {
                // [R]oll
                Action::Roll => match self.roll() {
                    0 | 7..=u32::MAX => panic!("outside dice bounds!"),
                    die @ 1 => {
                        println!("[DICE] Dice result is: {:3}!", die);
                        println!("[DUMP] Dumping Score! Sorry!");
                        self.status = TurnStatus::End;
                        break 'player 0;
                    }
                    die @ 2..=6 => {
                        println!("[DICE] Dice result is: {:3}!", die);
                        println!("[ROLL] Total    Score: {:3}!", (score + die));
                        println!("[HOLD] Possible Score: {:3}!\n", (score + die + self.score));
                        self.status = TurnStatus::Continue;
                        score += die
                    }
                },
                // [H]old
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

type DiceRoll = u32;
trait Dice {
    fn rng(&self) -> rand::rngs::ThreadRng;
    fn roll(&self) -> DiceRoll;
}

impl Dice for Player<'_> {
    fn rng(&self) -> rand::rngs::ThreadRng {
        rand::thread_rng()
    }

    fn roll(&self) -> DiceRoll {
        let sides = rand::distributions::Uniform::new(1, 6);
        self.rng().sample(sides)
    }
}

struct Stage<'play> {
    players: Vec<Player<'play>>,
}

impl<'play> Stage<'_> {
    fn new(player_a: Player<'play>, player_b: Player<'play>) -> Stage<'play> {
        Stage {
            players: vec![player_a, player_b],
        }
    }

    fn perform(&mut self) {
        'game: loop {
            for player in &mut self.players {
                if player.score <= 100 || player.status == TurnStatus::Continue {
                    println!("{} has {:?} Score", player.name, player.score);
                    player.resolve();
                } else {
                    println!("{} wins!", player.name);
                    break 'game;
                }
            }
        }
    }
}
