//! This module provides interface functionalities and manages all the input/output part of the program

use std::string::String;
use std::io::{self, Write};
use reversi::Side;
use reversi::board::{BOARD_SIZE, Coord};
use reversi::game::PlayerAction;
use reversi::turn::{State, Turn};
use ::{Result, Action, OtherAction};
use termion::{color, style};

// ANSI version
const DARK_DISK: char = '@';
const LIGHT_DISK: char = '@';
const EMPTY_CELL: char = '*';
const LEGAL_MOVE: char = '*';

pub enum UserCommand {
    NewGame,
    HumanPlayer,
    AiWeak,
    AiMedium,
    AiStrong,
    Help,
    Credits,
    Quit,
}

const COLUMN_WIDTH: u8 = 25;

fn ruler() {
    let mut ruler = "\t".to_string();
    for _ in 0..COLUMN_WIDTH {
        ruler.push('-');
    }
    println!("{}", ruler);
}

fn header(title: &str) {
    print!("\n\n\n {}", style::Bold);
    ruler();
    let middle = (COLUMN_WIDTH - title.len() as u8 - 2) / 2;
    let mut header = "\t".to_string();
    for _ in 0..middle{
        header.push('-');
    }
    header.push_str(&format!(" {} ", title));
    for _ in (middle + title.len() as u8 + 2)..COLUMN_WIDTH {
        header.push('-');
    }
    println!("{}", header);
    ruler();
    print!("{}", style::Reset);
}

const INTRO: &'static str =
"\t  a simple Reversi game
\twritten in Rust with love
\t      Redox Edition";

pub fn intro() {
    header("RUSThello");
    println!("{}", INTRO);
    println!("\t        v. 2.0.0");
}

const MAIN_MENU: &'static str =
"\tn - New match
\th - Help
\tc - Credits
\tq - Quit RUSThello";

pub fn main_menu() {
    header("MAIN MENU");
    println!("{}", MAIN_MENU);
    ruler();
}

const NEW_PLAYER_MENU: &'static str =
"\th - Human Player
\tw - Weak   AI
\tm - Medium AI
\ts - Strong AI
\tq - Quit match";

pub fn new_player_menu() {
    header("CHOOSE PLAYER");
    println!("{}", NEW_PLAYER_MENU);
    ruler();
}

const COMMANDS_INFO: &'static str = "\n\n
\tStarting new game...
\tType a cell's coordinates to place your disk there.
\tExaple: \"c4\" (or \"C4\", \"4c\", \"4C\", etc...).
\tType 'help' or 'h' to display a help message.
\tType 'undo' or 'u' to undo the last move.
\tType 'quit' or 'q' to abandon the game.";

pub fn commands_info() {
    println!("{}", COMMANDS_INFO);
}

const HELP: &'static str =
"\tReversi is a board game where two players compete against each other. The game is played \
on a 8x8 board, just like chess but for the squares’ colour which is always green. There are \
64 identical pieces called disks, which are white on one side and black on the other. A \
player is Dark, using disks’ black side, and the other one is Light, using disks' white \
side. The game starts with four disks already placed at the centre of the board, two for \
each side. Dark moves first.\n
\tLet’s say it’s Dark’s turn, for simplicity's sake, as for \
Light the rules are just the same. Dark has to place a disk in a free square of the board, \
with the black side facing up. Whenever the newly placed black disk and any other previously \
placed black disk enclose a sequence of white disks (horizontal, vertical or diagonal and of \
any length), all of those flip and turn black. It is mandatory to place the new disk such \
that at least a white disk is flipped, otherwise the move is not valid.\n
\tUsually players’ \
turn alternate, passing from one to the other. When a player cannot play any legal move, the \
turn goes back to the other player, thus allowing the same player to play consecutive turns. \
When neither player can play a legal move, the game ends. Usually, this happens when the \
board is completely filled up with disks (for a total of 60 turns). Games also happen \
sometimes to end before that, leaving empty squares on the board.\n
\tWhen the game ends, \
the player with more disks turned to its side wins. Ties are possible as well, if both \
player have the same number of disks.";

const RUSTHELLO: &'static str =
"\tTo play RUSThello you first have to choose who is playing on each side, Dark and Light. You can \
choose a human players or an AI. Choose human for both players and challenge a friend, or \
test your skills against an AI, or even relax as you watch two AIs competing against each \
other: all combinations are possible!\n
\tAs a human player, you move by entering the \
coordinates (a letter and a number) of the square you want to place your disk on, e.g. all \
of 'c4', 'C4', '4c' and '4C' are valid and equivalent coordinates. For your ease of use, all \
legal moves are marked on the board by an asterisk.\n
\tFurthermore, on your turn you can \
also input special commands: 'undo' (or 'u') to undo your last move (and yes, you can 'undo' \
as many times as you like), 'help' (or 'h') to see this help message again, and 'quit' (or \
'q') to quit the game.";

pub fn help() {
    header("REVERSI");
    println!("{}", HELP);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        panic!("\tFailed to read input!");
    }
    header("RUSThello");
    println!("{}", RUSTHELLO);
}

pub fn credits() {
    header("CREDITS");
    println!("\tRUSThello v. 2.0.0 Redox Edition");
    println!("\tby Enrico Ghiorzi,\n\twith the invaluable help of the Redox community");
    println!("\tCopyright (c) 2015-2016 by Enrico Ghiorzi");
    println!("\tReleased under the MIT license");
}

/// Reads user's input
fn get_user_input() -> String {
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        panic!("\tFailed to read input!");
    }
    input = input.trim().to_lowercase();
    input
}

/// It gets an input from the user and tries to parse it, then returns a Option<UserCommand>`.
/// If the input is recognized as a legit command, it returns the relative `Option::Some(UserCommand)`.
/// If the input is not recognized as a legit command, it returns a `Option::None`.
pub fn input_main_menu() -> UserCommand {
    print!("\tInsert input: ");
    loop {
        match &*get_user_input() {
            "n" | "new game" => return UserCommand::NewGame,
            "h" | "help" => return UserCommand::Help,
            "c" | "credits" => return UserCommand::Credits,
            "q" | "quit" | "exit" => return UserCommand::Quit,
            _ => {
                print!("\tInvalid command! Try again: ");
                continue;
            }
        }
    }
}

pub fn choose_new_player(side: Side) -> UserCommand {
    match side {
        Side::Dark => print!("\t{}Dark{}  player: ", style::Bold, style::Reset),
        Side::Light => print!("\t{}Light{} player: ", style::Bold, style::Reset),
    }
    loop {
        match &*get_user_input() {
            "h" | "human" | "player" | "human player" => return UserCommand::HumanPlayer,
            "w" | "weak" | "weak ai" => return UserCommand::AiWeak,
            "m" | "medium" | "medium ai" => return UserCommand::AiMedium,
            "s" | "strong" | "strong ai" => return UserCommand::AiStrong,
            "q" | "quit" | "exit" => return UserCommand::Quit,
            _ => {
                print!("\tInvalid command! Try again: ");
                continue;
            }
        }
    }
}

/// It get_status a human player's input and convert it into a move.
/// If the move if illegal, it ask for another input until the given move is a legal one.
pub fn human_make_move(turn: &Turn) -> Result<Action> {

    if let Some(side) = turn.get_state() {
        match side {
            Side::Dark => print!("\t{}Dark{}  moves: ", style::Bold, style::Reset),
            Side::Light => print!("\t{}Light{} moves: ", style::Bold, style::Reset),
        }
    } else {
        unreachable!();
    }

    loop {
        let input = &*get_user_input();
        match input {
            "h" | "help" => return Ok(PlayerAction::Other(OtherAction::Help)),
            "u" | "undo" => return Ok(PlayerAction::Undo),
            "q" | "quit" => return Ok(PlayerAction::Other(OtherAction::Quit)),
            _other_input => {
                let mut row: Option<usize> = None;
                let mut col: Option<usize> = None;

                for curr_char in input.chars() {
                    match curr_char {
                        '1'...'8' => row = Some(curr_char as usize - '1' as usize),
                        'a'...'h' => col = Some(curr_char as usize - 'a' as usize),
                        _ => {}
                    }
                }

                if row.is_none() || col.is_none() {
                    print!("\tIllegal move, try again: ");
                    continue;
                } else {
                    let coord = Coord::new(row.unwrap(), col.unwrap());
                    if turn.check_move(coord).is_ok() {
                        return Ok(PlayerAction::Move(coord));
                    } else {
                        print!("\tIllegal move, try again: ");
                        continue;
                    }
                }
            }
        }
    }
}

/// draw_board draws the board (using text characters) in a pleasant-looking way, converting the board in a string (board_to_string) and then printing this.
pub fn draw_board(turn: &Turn) {
    let board = turn.get_board();
    // Declare board_to_string and add column reference at the top
    let mut board_to_string: String = format!("\n\t{}                         {}\n", color::Bg(color::LightGreen), color::Bg(color::Reset));
    board_to_string.push_str(&format!("\t{}{}     A B C D E F G H     {}{}\n", color::Bg(color::LightGreen), color::Fg(color::Black), color::Fg(color::Reset), color::Bg(color::Reset)));

    // For every row…
    for row in 0..BOARD_SIZE {
        // Add a row reference to the left
        board_to_string.push_str(&format!("\t{}  {}{}{} {}", color::Bg(color::LightGreen), color::Fg(color::Black), row + 1, color::Fg(color::Reset), color::Bg(color::Reset)));
        // Set background color to green
        board_to_string.push_str(&format!("{} ", color::Bg(color::Green)));
        // For every column, add the appropriate character depending on the content of the current cell
        for col in 0..BOARD_SIZE {
            let coord = Coord::new(row, col);
            board_to_string.push_str(
	            &match board.get_cell(coord).unwrap() {
	                // Light and Dark cells are represented by white and black bullets
	                Some(disk) => match disk.get_side() {
						Side::Dark  => format!("{}{}{}", color::Fg(color::Black), DARK_DISK, color::Fg(color::Reset)),
						Side::Light => format!("{}{}{}", color::Fg(color::LightWhite), LIGHT_DISK, color::Fg(color::Reset)),
					},
	                // An empty cell will display a plus or a multiplication sign if the current player can move in that cell
	                // or a little central dot otherwise
	                None => {
						if turn.check_move(coord).is_ok() {
                            match turn.get_state() {
                                Some(Side::Dark)  => format!("{}{}{}{}{}", color::Fg(color::LightBlack), style::Faint, LEGAL_MOVE, style::NoFaint, color::Fg(color::Reset)),
                                Some(Side::Light) => format!("{}{}{}{}{}", color::Fg(color::White), style::Faint, LEGAL_MOVE, style::NoFaint, color::Fg(color::Reset)),
                                None => panic!("This should never happen!")
                            }
	                    } else {
							format!("{}{}{}", color::Fg(color::LightGreen), EMPTY_CELL, color::Fg(color::Reset))
	                	}
					}
	            }
			);
            board_to_string.push(' ');
        }
        // Reset background color
        board_to_string.push_str(&format!("{}", color::Bg(color::Reset)));

        // Add a row reference to the right
        board_to_string.push_str(&format!("{} {}{}{}  {}\n", color::Bg(color::LightGreen), color::Fg(color::Black), row + 1, color::Fg(color::Reset), color::Bg(color::Reset)));
    }

    // Add column reference at the bottom
    board_to_string.push_str(&format!("\t{}{}     A B C D E F G H     {}{}", color::Bg(color::LightGreen), color::Fg(color::Black), color::Fg(color::Reset), color::Bg(color::Reset)));

    // Print board
    println!("{}", board_to_string);

    // Print current score and game info
    let (score_dark, score_light) = turn.get_score();
    println!("\t{}                         {}", color::Bg(color::LightGreen), color::Bg(color::Reset));
    print!("\t{}{}       {:>2}{} ", color::Bg(color::LightGreen), color::Fg(color::Black), score_dark, color::Fg(color::Reset));
    match turn.get_state() {
        Some(side) => match side {
            Side::Dark => {
                print!("{}{}{}{}{}   {}{}{}",
                    color::Fg(color::Black), style::Blink, DARK_DISK, style::NoBlink, color::Fg(color::Reset),
                    color::Fg(color::LightWhite), LIGHT_DISK, color::Fg(color::Reset));
            }
            Side::Light => {
                print!("{}{}{}   {}{}{}{}{}",
                    color::Fg(color::Black), DARK_DISK, color::Fg(color::Reset),
                    color::Fg(color::LightWhite), style::Blink, LIGHT_DISK, style::NoBlink, color::Fg(color::Reset));
            }
        },
        None => {
            print!("{}{}{}   {}{}{}",
                color::Fg(color::Black), DARK_DISK, color::Fg(color::Reset),
                color::Fg(color::LightWhite), LIGHT_DISK, color::Fg(color::Reset));
        }
    };
    println!(" {}{:<2}       {}{}\n", color::Fg(color::Black), score_light, color::Fg(color::Reset), color::Bg(color::Reset));
}

/// Prints a message with info on a move.
pub fn move_message(side: Side, coord: Coord) {
    let char_col = (('a' as u8) + (coord.get_col() as u8)) as char;
    match side {
        Side::Dark => {
            println!("\t{}Dark{}  moves: {}{}",
                style::Bold, style::Reset,
                char_col,
                coord.get_row() + 1)
        }
        Side::Light => {
            println!("\t{}Light{} moves: {}{}",
                style::Bold, style::Reset,
                char_col,
                coord.get_row() + 1)
        }
    }
}

/// Print a message to declare winner
pub fn endgame_message(winner: Option<Side>) {
    match winner {
        Some(Side::Dark) => println!("\t{}Dark wins{}!", style::Bold, style::Reset),
        Some(Side::Light) => println!("\t{}Light wins{}!", style::Bold, style::Reset),
        None => println!("\t{}Tie{}!", style::Bold, style::Reset),
    }

}

/// Print a last message before a player quits the game
pub fn quitting_message(state: State) {
    match state {
        Some(Side::Dark) => println!("\t{}Dark{} is running away, the coward!", style::Bold, style::Reset),
        Some(Side::Light) => println!("\t{}Light{} is running away, the coward!", style::Bold, style::Reset),
        None => println!("\n\t{}Goodbye!{}", style::Bold, style::Reset),
    }
}

/// Print a last message when 'undo' is not possible
pub fn no_undo_message(undecided: Side) {
    match undecided {
        Side::Dark => println!("\tThere is no move {}Dark{} can undo.", style::Bold, style::Reset),
        Side::Light => println!("\tThere is no move {}Light{} can undo.", style::Bold, style::Reset),
    }
}
