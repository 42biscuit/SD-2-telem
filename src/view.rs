use egui::{Context, Ui};

use crate::{data::Data, graph::Graph};
///
/// Colletion of graphs, front end only
///
pub struct View<'a> {
    pub rows: Vec<Vec<usize>>,
    pub graphs: Vec<Box<dyn Graph<'a> + 'a>>,
}

impl<'a> View<'a> {
    ///New empty graphs

    pub fn new() -> View<'a> {
        View { rows: Vec::new(), graphs: Vec::new() }
    }

    pub fn add_graph(&mut self, row: usize, graph: Box<dyn Graph<'a> + 'a>) {
        while self.rows.len() < row + 1 {
            self.rows.push(Vec::new());
        }
        self.rows[row].push(self.graphs.len());
        self.graphs.push(graph);
    }

    pub fn draw(&self, data: &Data, ctx: &Context, ui: &mut Ui) {
        for row in &self.rows {
            ui.horizontal(|ui| {
                for graph_i in row {
                    self.graphs[*graph_i].draw(data, ctx, ui);
                }
            });
        }
    }
}
