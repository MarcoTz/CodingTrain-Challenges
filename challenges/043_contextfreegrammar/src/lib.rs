use graphics::{
    colors::Rgba, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    Updatable, UpdateContext, WindowConfig,
};
use piston::{Button, ButtonState, Key};
use piston_window::text::Text;
use std::fmt;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const FONT_SIZE: u32 = 18;
const UPDATE_TIME: f64 = 0.1;

type Variable = String;
type Terminal = String;

#[derive(Clone)]
enum VarOrTerm {
    Var(Variable),
    Term(Terminal),
}

impl fmt::Display for VarOrTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarOrTerm::Var(s) => f.write_str(&s),
            VarOrTerm::Term(s) => f.write_str(&s),
        }
    }
}

impl VarOrTerm {
    fn var(s: &str) -> VarOrTerm {
        VarOrTerm::Var(s.to_owned())
    }

    fn term(s: &str) -> VarOrTerm {
        VarOrTerm::Term(s.to_owned())
    }
}

type Rule = (Variable, Vec<VarOrTerm>);

struct Grammar {
    variables: Vec<Variable>,
    terminals: Vec<Terminal>,
    rules: Vec<Rule>,
    start: Variable,
}

impl Grammar {
    fn vars_from_strs(strs: &[&str]) -> Vec<Variable> {
        strs.iter().map(|s| (*s).to_owned()).collect()
    }

    fn terms_from_strs(strs: &[&str]) -> Vec<Terminal> {
        strs.iter().map(|s| (*s).to_owned()).collect()
    }

    fn rules_to_str(&self) -> String {
        let mut out = "".to_owned();
        for rule in self.rules.iter() {
            out += &rule.0;
            out += " -> ";
            let mut rhs = rule
                .1
                .iter()
                .map(|vt| vt.to_string())
                .collect::<Vec<String>>()
                .join("");
            if rhs.is_empty() {
                rhs = "eps".to_owned();
            }
            out += &rhs;
            out += " | ";
        }
        out
    }

    pub fn num_grammars() -> usize {
        6
    }

    pub fn ith(i: usize) -> Grammar {
        let num_grammars = Self::num_grammars();
        let i = i % num_grammars;
        let mut grammars = vec![
            Self::word_reverse(),
            Self::well_formed_parens(),
            Self::well_formed_parens_brack(),
            Self::match_pairs(),
            Self::dist_ab(),
            Self::double_b(),
        ];
        grammars.remove(i)
    }

    pub fn word_reverse() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S"]),
            terminals: Self::terms_from_strs(&["a", "b"]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("a"),
                        VarOrTerm::var("S"),
                        VarOrTerm::term("a"),
                    ],
                ),
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("b"),
                        VarOrTerm::var("S"),
                        VarOrTerm::term("b"),
                    ],
                ),
                ("S".to_owned(), vec![VarOrTerm::term("")]),
            ],
            start: "S".to_owned(),
        }
    }

    pub fn well_formed_parens() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S"]),
            terminals: Self::terms_from_strs(&["(", ")"]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![VarOrTerm::var("S"), VarOrTerm::var("S")],
                ),
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("("),
                        VarOrTerm::var("S"),
                        VarOrTerm::term(")"),
                    ],
                ),
                (
                    "S".to_owned(),
                    vec![VarOrTerm::term("("), VarOrTerm::term(")")],
                ),
            ],
            start: "S".to_owned(),
        }
    }

    pub fn well_formed_parens_brack() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S"]),
            terminals: Self::terms_from_strs(&["(", ")", "[", "]"]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![VarOrTerm::var("S"), VarOrTerm::var("S")],
                ),
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("("),
                        VarOrTerm::var("S"),
                        VarOrTerm::term(")"),
                    ],
                ),
                (
                    "S".to_owned(),
                    vec![VarOrTerm::term("("), VarOrTerm::term(")")],
                ),
                (
                    "S".to_owned(),
                    vec![VarOrTerm::term("["), VarOrTerm::term("]")],
                ),
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("["),
                        VarOrTerm::var("S"),
                        VarOrTerm::term("]"),
                    ],
                ),
            ],
            start: "S".to_owned(),
        }
    }

    pub fn match_pairs() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S"]),
            terminals: Self::terms_from_strs(&["a", "b"]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("a"),
                        VarOrTerm::var("S"),
                        VarOrTerm::term("b"),
                    ],
                ),
                (
                    "S".to_owned(),
                    vec![VarOrTerm::term("a"), VarOrTerm::term("b")],
                ),
            ],
            start: "S".to_owned(),
        }
    }

    pub fn dist_ab() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S", "T", "U", "V"]),
            terminals: Self::terms_from_strs(&["a", "b", ""]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![VarOrTerm::var("T"), VarOrTerm::var("U")],
                ),
                (
                    "T".to_owned(),
                    vec![
                        VarOrTerm::var("V"),
                        VarOrTerm::term("a"),
                        VarOrTerm::var("T"),
                    ],
                ),
                (
                    "T".to_owned(),
                    vec![
                        VarOrTerm::var("V"),
                        VarOrTerm::term("a"),
                        VarOrTerm::var("V"),
                    ],
                ),
                (
                    "T".to_owned(),
                    vec![
                        VarOrTerm::var("T"),
                        VarOrTerm::term("a"),
                        VarOrTerm::var("V"),
                    ],
                ),
                (
                    "U".to_owned(),
                    vec![
                        VarOrTerm::var("V"),
                        VarOrTerm::term("b"),
                        VarOrTerm::var("U"),
                    ],
                ),
                (
                    "U".to_owned(),
                    vec![
                        VarOrTerm::var("V"),
                        VarOrTerm::term("b"),
                        VarOrTerm::var("V"),
                    ],
                ),
                (
                    "U".to_owned(),
                    vec![
                        VarOrTerm::var("U"),
                        VarOrTerm::term("b"),
                        VarOrTerm::var("V"),
                    ],
                ),
                (
                    "V".to_owned(),
                    vec![
                        VarOrTerm::term("a"),
                        VarOrTerm::var("V"),
                        VarOrTerm::term("b"),
                        VarOrTerm::var("V"),
                    ],
                ),
                (
                    "V".to_owned(),
                    vec![
                        VarOrTerm::term("b"),
                        VarOrTerm::var("V"),
                        VarOrTerm::term("a"),
                        VarOrTerm::var("V"),
                    ],
                ),
                ("V".to_owned(), vec![VarOrTerm::term("")]),
            ],
            start: "S".to_owned(),
        }
    }

    fn double_b() -> Grammar {
        Grammar {
            variables: Self::vars_from_strs(&["S", "A"]),
            terminals: Self::terms_from_strs(&["a", "b"]),
            rules: vec![
                (
                    "S".to_owned(),
                    vec![
                        VarOrTerm::term("b"),
                        VarOrTerm::var("S"),
                        VarOrTerm::term("b"),
                        VarOrTerm::term("b"),
                    ],
                ),
                ("S".to_owned(), vec![VarOrTerm::var("A")]),
                (
                    "A".to_owned(),
                    vec![VarOrTerm::term("a"), VarOrTerm::var("A")],
                ),
                ("A".to_owned(), vec![VarOrTerm::term("")]),
            ],
            start: "S".to_owned(),
        }
    }
}

pub struct ContextfreeGrammar {
    current_grammar: usize,
    grammar: Grammar,
    output: Vec<VarOrTerm>,
    var_color: Rgba,
    term_color: Rgba,
    rule_color: Rgba,
    output_color: Rgba,
    ticks: f64,
}

impl ContextfreeGrammar {
    pub fn new() -> ContextfreeGrammar {
        let grammar = Grammar::ith(0);
        ContextfreeGrammar {
            current_grammar: 0,
            output: vec![VarOrTerm::Var(grammar.start.clone())],
            grammar,
            var_color: Rgba::random(),
            term_color: Rgba::random(),
            rule_color: Rgba::random(),
            output_color: Rgba::random(),
            ticks: 0.0,
        }
    }
}

impl Drawable for ContextfreeGrammar {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let start_x = FONT_SIZE as f64;
        let y_diff = 2.0 * FONT_SIZE as f64;
        let mut y = FONT_SIZE as f64;

        let vars_str = self.grammar.variables.join(" | ");
        Text::new_color(self.var_color.into(), FONT_SIZE)
            .draw_pos(
                &vars_str,
                [start_x, y],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();

        y += y_diff;
        let term_str = self.grammar.terminals.join(" | ");
        Text::new_color(self.term_color.into(), FONT_SIZE)
            .draw_pos(
                &term_str,
                [start_x, y],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();

        y += y_diff;
        let rules_str = self.grammar.rules_to_str();
        Text::new_color(self.rule_color.into(), FONT_SIZE)
            .draw_pos(
                &rules_str,
                [start_x, y],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();

        let out_str = self
            .output
            .iter()
            .map(|vt| vt.to_string())
            .collect::<Vec<String>>()
            .join("");
        Text::new_color(self.output_color.into(), 2 * FONT_SIZE)
            .draw_pos(
                &out_str,
                [
                    ctx.args.window_size[0] / 2.0 - self.output.len() as f64 * FONT_SIZE as f64,
                    ctx.args.window_size[1] / 2.0,
                ],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
    }
}

impl Updatable for ContextfreeGrammar {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.ticks += ctx.args.dt;
        if self.ticks < UPDATE_TIME {
            return;
        }
        self.ticks = 0.0;

        let mut next_out = vec![];
        let out_len = self.output.len();
        for _ in 0..out_len {
            let next_ind = self.output.len() - 1;
            let next_vt = self.output.remove(next_ind);
            match next_vt {
                VarOrTerm::Term(_) => next_out.insert(0, next_vt),
                VarOrTerm::Var(v) => {
                    let possible_rules: Vec<&Rule> = self
                        .grammar
                        .rules
                        .iter()
                        .filter(|rule| rule.0 == v)
                        .collect();
                    let appl_rule = rand::random::<usize>() % possible_rules.len();
                    for vt in possible_rules[appl_rule].1.iter().rev() {
                        next_out.insert(0, vt.clone());
                    }
                }
            }
        }
        self.output = next_out;
    }
}

impl EventHandler for ContextfreeGrammar {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release {
            return;
        }

        if ctx.args.button == Button::Keyboard(Key::N) {
            self.current_grammar += 1;
            self.current_grammar = self.current_grammar % Grammar::num_grammars();
            self.grammar = Grammar::ith(self.current_grammar);
        }

        self.output = vec![VarOrTerm::Var(self.grammar.start.clone())]
    }
}

impl Runnable for ContextfreeGrammar {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Context-Free Grammar".to_owned(),
        }
    }
}
