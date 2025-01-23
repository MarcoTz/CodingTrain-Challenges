use graphics_lib::app::App;
use std::env;

const ERR_EXIT_MSG: &str = "No number provided, try --help for help";
const HELP_MSG: &str = "Usage: runner [NUMBER]\nRuns challenge NUMBER";

fn run_app(args: &mut env::Args) {
    // This is always the current challenge
    if args.len() == 1 {
        return App::new(reactiondiffusion::ReactionDiffusion::new()).run();
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
        1 => App::new(starfield::StarSpawner::new()).run(),
        2 => App::new(mengersponge::Menger::new()).run(),
        3 => App::new(snake::SnakeGame::new()).run(),
        4 => App::new(purplerain::RainCloud::new()).run(),
        5 => App::new(spaceinvaders::SpaceInvaders::new()).run(),
        6 => App::new(mitosis::Mitosis::new()).run(),
        7 => App::new(solarsystem::SolarSystem::new()).run(),
        8 | 9 => panic!("3D solar system (with or without textures not implemented"),
        10 => App::new(mazegenerator::MazeGenerator::new()).run(),
        11 => panic!("3D terrain generation is not implemented"),
        12 => panic!("Lorentz attractor is not implemented"),
        13 => App::new(reactiondiffusion::ReactionDiffusion::new()).run(),
        _ => panic!("Challenge {num} does not exist"),
    }
}

fn main() {
    let mut args = env::args();
    run_app(&mut args);
}
