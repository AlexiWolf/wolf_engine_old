use std::sync::Arc;

use crate::ecs::*;
use crate::events::*;
use crate::EventLoop;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Under the hood, Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// "the engine."
///
/// The Context owns all engine data, sub-systems, and the link to the Event-Loop through which
/// all events are sent.  As far as the end-user is concerned, the Context *is* the engine.
pub struct Context<E: UserEvent> {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
    render_schedule: Schedule,
    event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    /// Create a new `Context` from the provided [`EventQueue`] and data.
    pub(crate) fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub fn update(&mut self) {
        self.update_schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn render(&mut self) {
        self.render_schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn update_schedule(&self) -> &Schedule {
        &self.update_schedule
    }

    pub fn update_schedule_mut(&mut self) -> &mut Schedule {
        &mut self.update_schedule
    }

    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }
}

impl<E: UserEvent> HasEventSender<Event<E>> for Context<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event<E>>> {
        self.event_sender.clone()
    }
}

pub(crate) struct ContextBuilder {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
    render_schedule: Schedule,
}

impl ContextBuilder {
    pub(crate) fn new() -> Self {
        Self {
            world: Default::default(),
            resources: Default::default(),
            update_schedule: Schedule::builder().build(),
            render_schedule: Schedule::builder().build(),
        }
    }

    pub fn with_resources(mut self, resources: Resources) -> Self {
        self.resources = resources;
        self
    }

    pub fn with_update_schedule(mut self, schedule: Schedule) -> Self {
        self.update_schedule = schedule;
        self
    }

    pub fn with_render_schedule(mut self, schedule: Schedule) -> Self {
        self.render_schedule = schedule;
        self
    }

    pub fn build<E: UserEvent>(self, event_loop: &EventLoop<E>) -> Context<E> {
        Context {
            world: self.world,
            resources: self.resources,
            update_schedule: self.update_schedule,
            render_schedule: self.render_schedule,
            event_sender: event_loop.event_sender(),
        }
    }
}

#[cfg(test)]
mod context_tests {
    #[test]
    fn should_run_ecs_tick() {
        #[crate::ecs::system]
        fn add_1(#[resource] number: &mut i32) {
            *number += 1;
        }
        let (_, mut context) = crate::init::<()>()
            .with_resources(|resources| {
                resources.add_resource(0);
            })
            .with_update_schedule(|schedule| {
                schedule.add_system(add_1_system());
            })
            .with_render_schedule(|schedule| {
                schedule.add_system(add_1_system());
            })
            .build();
        
        assert_eq!(*context.resources().get::<i32>().unwrap(), 0);
        context.update();
        assert_eq!(*context.resources().get::<i32>().unwrap(), 1);
        context.render();
        assert_eq!(*context.resources().get::<i32>().unwrap(), 2);

    }

    #[test]
    fn should_have_accessors() {
        let (_, mut context) = crate::init::<()>().build();
        {
            let _world = context.world();
            let _resources = context.resources();
            let _update_schedule = context.update_schedule();
            let _render_schedule = context.render_schedule();
        }
        {
            let _world_mut = context.world_mut();
            let _mut_resources = context.resources_mut();
            let _update_schedule = context.update_schedule_mut();
            let _render_schedule = context.render_schedule_mut();
        }
    }
}
