use super::{
    algae::Algae, bin_tree::BinTree, cantor::Cantor, dragon::Dragon, koch::Koch, l_system::LSystem,
    plant::Plant, sierpinski::Sierpinski, sierpinski_curve::SierpinskiCurve,
    turtle::TurtleInstructor,
};
use graphics_lib::{Updatable, UpdateContext};

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
    pub fn next(self) -> Self {
        match self {
            System::Algae(_, _) => {
                let system = BinTree::l_system();
                System::BinTree(system.axiom.clone(), system)
            }
            System::BinTree(_, _) => {
                let system = Cantor::l_system();
                System::Cantor(system.axiom.clone(), system)
            }
            System::Cantor(_, _) => {
                let system = Dragon::l_system();
                System::Dragon(system.axiom.clone(), system)
            }

            System::Dragon(_, _) => {
                let system = Koch::l_system();
                System::Koch(system.axiom.clone(), system)
            }
            System::Koch(_, _) => {
                let system = Plant::l_system();
                System::Plant(system.axiom.clone(), system)
            }
            System::Plant(_, _) => {
                let system = Sierpinski::l_system();
                System::Sierpinski(system.axiom.clone(), system)
            }
            System::Sierpinski(_, _) => {
                let system = SierpinskiCurve::l_system();
                System::SierpinskiCurve(system.axiom.clone(), system)
            }
            System::SierpinskiCurve(_, _) => {
                let system = Algae::l_system();
                System::Algae(system.axiom.clone(), system)
            }
        }
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

    pub fn next_iter(&mut self) -> Vec<Box<dyn TurtleInstructor>> {
        match self {
            System::Algae(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::BinTree(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::Cantor(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::Dragon(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::Koch(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::Plant(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::Sierpinski(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
            System::SierpinskiCurve(st, sys) => {
                *st = sys.next(st);
                st.clone()
                    .into_iter()
                    .map(|a| Box::new(a) as Box<dyn TurtleInstructor>)
                    .collect()
            }
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
    fn update(&mut self, _: &UpdateContext) {
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
