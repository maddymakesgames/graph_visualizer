use egui::{ComboBox, Ui};

use crate::{
    app::GraphApp,
    generation::{generators, GraphGenerator, GENERATOR_COUNT},
    menus::Menu,
};

pub struct GraphMenu {
    curr_generator: usize,
    generators: [Box<dyn GraphGenerator>; GENERATOR_COUNT],
}

impl Menu for GraphMenu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut Ui) {
        let curr_graph = app.curr_graph;
        let graphs = &mut app.graphs;

        let mut graph_selection = if graphs.is_empty() { 0 } else { curr_graph + 1 };
        ui.horizontal(|ui| {
            ui.label("Selected Graph");
            ComboBox::new("Selected Graph", "").show_index(
                ui,
                &mut graph_selection,
                graphs.len() + 1,
                |i| {
                    if i == 0 {
                        "Select Graph".to_owned()
                    } else {
                        graphs[i - 1].get_name()
                    }
                },
            );

            if ui.button("Remove Graph").clicked() {
                graphs.remove(curr_graph);
                if curr_graph > 0 {
                    app.curr_graph -= 1;
                }
            }
        });

        if graph_selection > 0 {
            app.curr_graph = graph_selection - 1;
        }

        ui.horizontal(|ui| {
            ui.label("Graph Generator");
            ComboBox::new("Graph Generator", "").show_index(
                ui,
                &mut self.curr_generator,
                self.generators.len(),
                |i| self.generators[i].name().to_owned(),
            );
        });

        ui.separator();

        let generator = &mut self.generators[self.curr_generator];

        generator.ui(ui);

        if ui.button("Generate Graph").clicked() {
            graphs.push(generator.gen_graph());
            app.curr_graph = graphs.len() - 1;
        }
    }

    fn name(&self) -> &'static str {
        "Graphs"
    }
}

impl Default for GraphMenu {
    fn default() -> Self {
        GraphMenu {
            curr_generator: 0,
            generators: generators(),
        }
    }
}
