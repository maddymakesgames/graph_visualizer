pub mod random;

use egui::{TextBuffer, TextEdit, Ui};

use crate::{generation::random::RandomGraphMenu, graph::Graph};

pub const GENERATOR_COUNT: usize = 2;

pub fn generators() -> [Box<dyn GraphGenerator>; GENERATOR_COUNT] {
    [
        Box::new(EmptyGraphGenerator::default()),
        Box::new(RandomGraphMenu::default()),
    ]
}

pub trait GraphGenerator {
    fn name(&self) -> &'static str;
    fn gen_graph(&mut self) -> Graph;
    fn ui(&mut self, ui: &mut Ui);
}

#[derive(Default)]
pub struct EmptyGraphGenerator {
    name: String,
    directed: bool,
    weighted: bool,
}

impl GraphGenerator for EmptyGraphGenerator {
    fn name(&self) -> &'static str {
        "Empty Graph"
    }

    fn gen_graph(&mut self) -> Graph {
        let graph = Graph::new(self.name.take(), self.directed, self.weighted);
        *self = Self::default();
        graph
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Graph Name:");
            TextEdit::singleline(&mut self.name).show(ui);
        });

        ui.checkbox(&mut self.directed, "Directed");
        ui.checkbox(&mut self.weighted, "Weighted");
    }
}
