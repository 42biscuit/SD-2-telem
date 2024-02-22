use egui::{Context, Ui};

use crate::{data::Data, graph::Graph};

pub struct View<'a> {
    pub graphs: Vec<Box<dyn Graph<'a> + 'a>>,
}

impl<'a> View<'a> {
    pub fn new() -> View<'a> {
        View {
            graphs: Vec::new(),
        }
    }

    pub fn add_graph(&mut self, graph: Box<dyn Graph<'a> + 'a>) {
        self.graphs.push(graph);
    }
    
    pub fn add_bar_chart(&mut self,chart : Box<dyn Graph<'a> + 'a>){
        self.graphs.push(chart);
    }

    pub fn draw(&self, data: &Data, ctx: &Context, ui: &mut Ui) {
        for graph in &self.graphs {
            graph.draw(data, ctx, ui);
        }
    }
}