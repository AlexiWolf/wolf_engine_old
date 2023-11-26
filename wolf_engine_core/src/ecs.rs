//! Provides an Entity-Component-System based on [Legion](::legion).

pub use legion::*;
pub use wolf_engine_codegen::system;

#[doc(hidden)]
pub mod prelude {
    pub use super::system;
    pub use super::ResourcesBuilder;
    pub use super::Schedule;
}

/// Provides a builder-pattern for creating [`Resources`].
#[derive(Default)]
pub struct ResourcesBuilder {
    resources: Resources,
}

impl ResourcesBuilder {
    /// Inserts the provide instance of `T` into the [`Resources`].
    ///
    /// If the provided type has previously been added, the existing instance is silently
    /// overwritten.
    /// This function is functionally-identical to calling [`Resources::insert()`]. pub fn add_resource<T: systems::Resource + 'static>(&mut self, resource: T) -> &mut Self {
    pub fn add_resource<T: systems::Resource + 'static>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    /// Consumes the builder, and returns the [`Resources`] from it.
    pub fn build(self) -> Resources {
        self.resources
    }
}
