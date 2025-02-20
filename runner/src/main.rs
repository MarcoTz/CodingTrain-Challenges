use graphics::app::App;
use std::env;

const ERR_EXIT_MSG: &str = "No number provided, try --help for help";
const HELP_MSG: &str = "Usage: runner [NUMBER]\nRuns challenge NUMBER";

fn run_app(args: &mut env::Args) {
    // This is always the current challenge
    if args.len() == 1 {
        return App::new(astar::AStar::new()).run();
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
        14 | 15 => App::new(fractaltrees::FractalTree::new()).run(),
        16 => App::new(lsystem::SystemRunner::new()).run(),
        17 => App::new(spacecolonization::SpaceColonization::new()).run(),
        18 => panic!("3D fractal trees are not implemented"),
        19 => App::new(supershape::SuperShape::new()).run(),
        20 => panic!("3D cloth is not implemented"),
        21 => App::new(mandelbrot::Mandelbrot::new()).run(),
        22 => App::new(juliaset::JuliaSet::new()).run(),
        23 => App::new(supershape::SuperShape::new()).run(),
        24 => App::new(perlinnoise::PerlinNoise::new()).run(),
        25 => panic!("Spherical Geometry not implemented"),
        26 => panic!("3D supershapes not implemented"),
        27 => App::new(fireworks::Fireworks::new()).run(),
        28 => App::new(metaballs::Metaballs::new()).run(),
        29 => App::new(smartrockets::SmartRockets::new()).run(),
        30 => App::new(phyllotaxis::Phyllotaxis::new()).run(),
        31 => App::new(flappybird::FlappyBird::new()).run(),
        32 => App::new(agario::Agario::new()).run(),
        33 => App::new(poissondisk::PoissonDisk::new()).run(),
        34 => App::new(diffusionlimitedaggregation::DiffAgg::new()).run(),
        35 => App::new(travellingsalesperson::TravellingSalesPerson::new()).run(),
        36 => App::new(blobby::Blobby::new()).run(),
        37 => App::new(diasticmachine::Diastic::new()).run(),
        38 => App::new(wordinteractor::WordInteractor::new()).run(),
        39 => App::new(madlibs::MadLibs::new()).run(),
        40 => App::new(tfidf::TfIdf::new()).run(),
        41 => panic!("Clappy Bird not implemented"),
        42 => App::new(markovchain::MarkovChain::new()).run(),
        43 => App::new(contextfreegrammar::ContextfreeGrammar::new()).run(),
        44 => App::new(afinn::Afinn::new()).run(),
        45 => panic!("Firebase not implemented"),
        46 => App::new(asteroids::Asteroids::new()).run(),
        47 => App::new(pixelsorting::PixelSorting::new()).run(),
        48 => panic!("White House social media data not implemented"),
        49 => panic!("White House social media mosaic not implemented"),
        50 => App::new(circlepacking::CirclePacking::new()).run(),
        51 => App::new(astar::AStar::new()).run(),
        _ => panic!("Challenge {num} does not exist"),
    }
}

fn main() {
    let mut args = env::args();
    run_app(&mut args);
}
