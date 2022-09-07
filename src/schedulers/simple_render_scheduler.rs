use crate::*;

pub struct SimpleRenderScheduler;

impl RenderScheduler for SimpleRenderScheduler {
    fn render(&mut self, context: &mut Context, state: &mut dyn State) {}
}
