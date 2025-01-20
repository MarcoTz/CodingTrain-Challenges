use graphics_lib::{app::App, Runnable};
use std::env;

const ERR_EXIT_MSG: &str = "No number provided, try --help for help";
const HELP_MSG: &str = "Usage: runner [NUMBER]\nRuns challenge NUMBER";

fn get_runnable(args: &mut env::Args) -> Box<dyn Runnable> {
    // This is always the current challenge
    if args.len() == 1 {
        return Box::new(snake::SnakeGame::new());
    }

    let arg = args.nth(1).expect(ERR_EXIT_MSG);
    let num = match arg.as_str() {
        "--help" | "-h" => {
            println!("{}", HELP_MSG);
            std::process::exit(0);
        }
        num => num.parse::<u64>().expect(ERR_EXIT_MSG),
    };

    match num {
        1 => Box::new(starfield::StarSpawner::new()),
        2 => Box::new(mengersponge::Menger::new()),
        3 => Box::new(snake::SnakeGame::new()),
        _ => panic!("Challenge {num} does not exist"),
    }
}

fn main() {
    let mut args = env::args();
    let runnable = get_runnable(&mut args);
    let mut app = App::new(runnable);
    app.run();
}
