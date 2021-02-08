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
 */

// TODO Need full reformatting of all information / outputs to make human-readable.
// TODO Use of loops are confusing. Entire thing is rough sketch -- how to clean?
use rand::prelude::*;

fn main() {
    println!("Beginning game of Pig...");
    let player1: Player = Player::new("PLAYER ONE (1)");
    let player2: Player = Player::new("PLAYER TWO (2)");

    let mut stage: Vec<Player> = vec![player1, player2];

    loop {
        for player in stage.iter_mut() {
            if player.score <= 100 || player.status != Status::End {
                println!("{} has {:?} Score", player.name, player.score);
                player._resolve();
            } else {
                println!("{} wins!", player.name);
                break;
            }
        }
    }
}

type DiceRoll = u32;

trait Dice {
    fn rng(&self) -> rand::rngs::ThreadRng;
    fn roll(&self) -> DiceRoll;
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Roll,
    Hold,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Status {
    Continue,
    End,
}

type Score = u32;
type Name<'name> = &'name str;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
struct Player<'name> {
    name: Name<'name>,
    score: Score,
    status: Status,
}

impl<'name> Player<'_> {
    fn new(name: Name) -> Player {
        Player {
            name,
            score: 0,
            status: Status::Continue,
        }
    }

    fn _input(&self) -> String {
        let mut cmd: String = String::new();
        std::io::stdin()
            .read_line(&mut cmd)
            .expect("could not read input");
        cmd.trim().parse().expect("could not configure input")
    }

    fn _action(&self) -> Action {
        // TODO Redo all of this fn() to make it cleaner.
        let input = self._input();
        let input = input
            .as_str()
            .to_lowercase()
            .chars()
            .next()
            .expect("could not retrieve first letter!");

        match input {
            'r' => Action::Roll,
            'h' => Action::Hold,
            _ => panic!("not a valid command!"),
        }
    }

    fn _turn(&mut self) -> Score {
        let mut score: Score = 0;
        loop {
            println!("[R]oll / [H]old");
            match self._action() {
                Action::Roll => match self.roll() {
                    0 | 7..=u32::MAX => panic!("outside dice bounds!"),
                    1 => {
                        println!("Dice result is (1)! Dumping score and ending turn...");
                        self.status = Status::End;
                        // TODO This is causing _resolve() to set score to 0?
                        break 0;
                    }
                    die @ 2..=6 => {
                        println!("Dice result is ({})!", die);
                        println!("Active Score is: [{}]!", (score + die));
                        println!("Held Score would be: [{}]\n", (score + die + self.score));
                        self.status = Status::Continue;
                        score += die
                    }
                },
                Action::Hold => {
                    self.status = Status::End;
                    break score;
                }
            }
        }
    }

    fn _resolve(&mut self) {
        self.score += self._turn()
    }
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
