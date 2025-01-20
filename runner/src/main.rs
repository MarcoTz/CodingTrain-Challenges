use std::env;

const ERR_EXIT_MSG: &str = "No number provided, try --help for help";
const HELP_MSG: &str = "Usage: runner [NUMBER]\nRuns challenge NUMBER";

fn main() {
    let mut args = env::args();

    // This is always the current challenge
    if args.len() == 1 {
        snake::run();
        return;
    }

    let arg = args.nth(1).expect(ERR_EXIT_MSG);
    let num = match arg.as_str() {
        "--help" | "-h" => {
            println!("{}", HELP_MSG);
            return;
        }
        num => num.parse::<u64>().expect(ERR_EXIT_MSG),
    };

    match num {
        1 => starfield::run(),
        2 => mengersponge::run(),
        3 => snake::run(),
        _ => panic!("Challenge {num} does not exist"),
    }
}
