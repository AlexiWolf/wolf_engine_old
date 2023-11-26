use std::sync::Arc;

use crate::ecs::*;
use crate::events::*;

/// Provides a container for Wolf Engine's user-facing data.
///
/// Wolf Engine consists of two main parts: The `Context` (You are here!), and the
/// [`EventLoop`](crate::EventLoop`).  Together, these two parts make up what we refer to as
/// ["the engine"](crate::Engine).
///
/// The Context owns all engine data, including resources, system schedules, and the game world.
pub struct Context<E: UserEvent> {
    pub(crate) world: World,
    pub(crate) resources: Resources,
    pub(crate) event_sender: Arc<dyn EventSender<Event<E>>>,
}

impl<E: UserEvent> Context<E> {
    pub fn run_schedule(&mut self, schedule: &mut Schedule) {
        schedule.execute(&mut self.world, &mut self.resources);
    }

    /// Returns an immutable reference to the world.
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Returns a mutable reference to the world.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Returns an immutable reference to engine resources.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Returns a mutable reference to engine resources.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    /// Sends a [Quit Event](Event::Quit) to trigger an engine shutdown.
    pub fn quit(&self) {
        self.event_sender.send_event(Event::Quit).ok();
    }
}

impl<E: UserEvent> HasEventSender<Event<E>> for Context<E> {
    fn event_sender(&self) -> Arc<dyn EventSender<Event<E>>> {
        self.event_sender.clone()
    }
}

#[cfg(test)]
mod context_tests {
    use legion::Schedule;

    use crate::ecs::ResourcesBuilder;

    #[test]
    fn should_run_ecs_tick() {
        #[legion::system]
        fn add_1(#[resource] number: &mut i32) {
            *number += 1;
        }
        let mut resources = ResourcesBuilder::default();
        resources.add_resource(0);
        let (_, mut context) = crate::init::<()>().with_resources(resources).build();

        let mut schedule = Schedule::builder().add_system(add_1_system()).build();

        assert_eq!(*context.resources().get::<i32>().unwrap(), 0);
        context.run_schedule(&mut schedule);
        assert_eq!(*context.resources().get::<i32>().unwrap(), 1);
        context.run_schedule(&mut schedule);
        assert_eq!(*context.resources().get::<i32>().unwrap(), 2);
    }

    #[test]
    fn should_have_accessors() {
        let (_, mut context) = crate::init::<()>().build();
        {
            let _world = context.world();
            let _resources = context.resources();
        }
        {
            let _world_mut = context.world_mut();
            let _mut_resources = context.resources_mut();
        }
    }
}
