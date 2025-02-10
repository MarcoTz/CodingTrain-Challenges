use super::{
    algae::Algae, bin_tree::BinTree, cantor::Cantor, dragon::Dragon, koch::Koch, l_system::LSystem,
    plant::Plant, sierpinski::Sierpinski, sierpinski_curve::SierpinskiCurve,
    turtle::TurtleInstructor,
};
use graphics::{Updatable, UpdateContext};
use std::fmt;

pub enum System {
    Algae(Vec<Algae>, LSystem<Algae>),
    BinTree(Vec<BinTree>, LSystem<BinTree>),
    Cantor(Vec<Cantor>, LSystem<Cantor>),
    Dragon(Vec<Dragon>, LSystem<Dragon>),
    Koch(Vec<Koch>, LSystem<Koch>),
    Plant(Vec<Plant>, LSystem<Plant>),
    Sierpinski(Vec<Sierpinski>, LSystem<Sierpinski>),
    SierpinskiCurve(Vec<SierpinskiCurve>, LSystem<SierpinskiCurve>),
}

impl System {
    pub fn all() -> Vec<System> {
        let mut systems = vec![];
        let system = BinTree::l_system();
        systems.push(System::BinTree(system.axiom.clone(), system));
        let system = Cantor::l_system();
        systems.push(System::Cantor(system.axiom.clone(), system));
        let system = Dragon::l_system();
        systems.push(System::Dragon(system.axiom.clone(), system));
        let system = Koch::l_system();
        systems.push(System::Koch(system.axiom.clone(), system));
        let system = Plant::l_system();
        systems.push(System::Plant(system.axiom.clone(), system));
        let system = Sierpinski::l_system();
        systems.push(System::Sierpinski(system.axiom.clone(), system));
        let system = SierpinskiCurve::l_system();
        systems.push(System::SierpinskiCurve(system.axiom.clone(), system));
        let system = Algae::l_system();
        systems.push(System::Algae(system.axiom.clone(), system));
        systems
    }

    pub fn axiom(&self) -> Vec<Box<dyn TurtleInstructor>> {
        match self {
            System::Algae(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::BinTree(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Cantor(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),

            System::Dragon(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),

            System::Koch(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),

            System::Plant(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),

            System::Sierpinski(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),

            System::SierpinskiCurve(_, sys) => sys
                .axiom
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
        }
    }

    pub fn reset(&mut self) {
        match self {
            System::Algae(st, sys) => *st = sys.axiom.clone(),
            System::BinTree(st, sys) => *st = sys.axiom.clone(),
            System::Cantor(st, sys) => *st = sys.axiom.clone(),
            System::Dragon(st, sys) => *st = sys.axiom.clone(),
            System::Koch(st, sys) => *st = sys.axiom.clone(),
            System::Plant(st, sys) => *st = sys.axiom.clone(),
            System::Sierpinski(st, sys) => *st = sys.axiom.clone(),
            System::SierpinskiCurve(st, sys) => *st = sys.axiom.clone(),
        }
    }

    pub fn next_iter(&mut self) {
        match self {
            System::Algae(st, sys) => {
                *st = sys.next(st);
            }
            System::BinTree(st, sys) => {
                *st = sys.next(st);
            }
            System::Cantor(st, sys) => {
                *st = sys.next(st);
            }
            System::Dragon(st, sys) => {
                *st = sys.next(st);
            }
            System::Koch(st, sys) => {
                *st = sys.next(st);
            }
            System::Plant(st, sys) => {
                *st = sys.next(st);
            }
            System::Sierpinski(st, sys) => {
                *st = sys.next(st);
            }
            System::SierpinskiCurve(st, sys) => {
                *st = sys.next(st);
            }
        }
    }

    pub fn commands(&self) -> Vec<Box<dyn TurtleInstructor>> {
        match self {
            System::Algae(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::BinTree(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Cantor(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Dragon(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Koch(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Plant(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::Sierpinski(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
            System::SierpinskiCurve(st, _) => st
                .clone()
                .into_iter()
                .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                .collect(),
        }
    }
}

impl Default for System {
    fn default() -> System {
        let system = Algae::l_system();
        System::Algae(system.axiom.clone(), system)
    }
}

impl Updatable for System {
    fn update(&mut self, _: &mut UpdateContext) {
        match self {
            System::Algae(st, sys) => *st = sys.next(st),
            System::BinTree(st, sys) => *st = sys.next(st),
            System::Cantor(st, sys) => *st = sys.next(st),
            System::Dragon(st, sys) => *st = sys.next(st),
            System::Koch(st, sys) => *st = sys.next(st),
            System::Plant(st, sys) => *st = sys.next(st),
            System::Sierpinski(st, sys) => *st = sys.next(st),
            System::SierpinskiCurve(st, sys) => *st = sys.next(st),
        }
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            System::Algae(_, _) => f.write_str("Algae"),
            System::BinTree(_, _) => f.write_str("BinTree"),
            System::Cantor(_, _) => f.write_str("Cantor"),
            System::Dragon(_, _) => f.write_str("Dragon"),
            System::Koch(_, _) => f.write_str("Koch"),
            System::Plant(_, _) => f.write_str("Plant"),
            System::Sierpinski(_, _) => f.write_str("Sierpinski"),
            System::SierpinskiCurve(_, _) => f.write_str("SierpinskiCurve"),
        }
    }
}
