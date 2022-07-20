//! Wolf Engine is a game framework designed to be flexible an easy to work with.
//!
//! **Note:** Wolf Engine is still a W.I.P. Many features are still missing and / or may change
//! without warning.  See the [README](https://github.com/Alexiwolf/wolf_engine#features) for more
//! information.
//!
//! # A Minimal Quick-Start Example
//!
//! To get started with Wolf Engine, add the following to your `Cargo.toml`.
//!
//! ```text
//! [dependencies]
//! wolf_engine = "*"
//! ```
//!
//! The [Engine] ships with sensible defaults to help you get up and running as quickly as possible.
//! For default settings, use [Engine::new()] or [Engine::default()], then run by calling
//! [Engine::run()] and passing your game's starting [State] to it.
//!
//! ```
//! use wolf_engine::*;
//! #
//! # use wolf_engine::utils::EngineControls;
//!
//! # #[allow(clippy::needless_doctest_main)]
//! pub fn main() {
//!     Engine::new()
//!         .run(Box::from(MyGameState));
//! }
//!
//! pub struct MyGameState;
//!
//! impl State for MyGameState {
//!     fn update(&mut self, context: &mut Context) -> OptionalTransition {
//!         // Update your game here.
//! #       context.quit();
//!         None
//!     }
//!
//!     fn render(&mut self, context: &mut Context) -> RenderResult {
//!         // Render your game here.
//!     }
//! }
//! ```
//!
//! # Important Concepts
//!
//! There are a few fundamental concepts you should understand in order to work effectively with
//! Wolf Engine.  They are as follows:
//!
//! - Building your Game using [States](State).
//! - Using the [StateStack] to run multiple [States](State).
//! - Storing and accessing global [Engine] data through the [Context] object.
//! - Using the [EngineBuilder] to configure the [Engine].
//! - Extending the [Engine] by loading [Plugins](Plugin).
//!
//! ## Game States and the State Stack
//!
//! Wolf Engine games are implemented as one or more [State] objects.  A [State] holds all the
//! logic and data for your game in a neat little package you can send to the [Engine].  When you
//! start the [Engine], you pass your game's [State] it.
//!
//!
//! An example of a simple game state:
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub struct MyState {
//!     counter: u64,
//! }
//!
//! impl State for MyState {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         self.counter += 1;
//!         None
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//! ```
//!
//! #### Multiple States
//!
//! It is possible to have more than one [State] loaded and running at any given time as the
//! [Engine] does not run the [States](State) directly.  Instead, the [State] is pushed onto the
//! [StateStack] in order to allow multiple [State]s to be loaded at the same time.  The
//! [StateStack] stores and runs all currently-loaded [State] objects for the [Engine], and is
//! controlled by [Transitions](Transition) returned from the [State::update()] method.
//!
//! When multiple [States](State) are loaded, they all run together from bottom-to-top order.  A
//! [State] which is below another [State] on the stack is considered to be a "background" or
//! "deactivated" [State] while the topmost [State] is designated as the "foreground" or "active"
//! [State].  A background [State] will only have its [State::background_update()] and
//! [State::background_render()] methods called while the active [State] will have its
//! [State::update()] and [State::render()] methods called.
//!
//! #### Changing States
//!
//! A [Transition] is ([Optionally](OptionalTransition)) returned by [State::update()] and is used
//! to control the [StateStack] by pushing and popping [States](State) on the [StateStack].
//!
//! ```
//! # use wolf_engine::*;
//! #
//! pub struct StateA {
//!     pub counter: u32,
//! }
//!
//! impl State for StateA {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         if self.counter < 10 {
//!             println!("Hello from State A!");
//!             // Increment the counter and push State B to the top of the stack.
//!             self.counter += 1;
//!             Some(Transition::Push(Box::from(StateB::new())))
//!         } else {
//!             // Once the counter reaches 10, pop all states off the stack.
//!             // An empty state stack will trigger an engine shutdown.
//!             Some(Transition::Clean)
//!         }
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//!
//! pub struct StateB {
//!     counter: u32,
//! }
//!
//! # impl StateB {
//! #   pub fn new() -> Self {
//! #       Self { counter: 0 }
//! #   }
//! # }
//! #
//! impl State for StateB {
//!     fn update(&mut self, _context: &mut Context) -> OptionalTransition {
//!         if self.counter < 3 {
//!             println!("Hello from State B!");
//!             // Increment the counter then return no transition.
//!             // The state will continue running as-is.
//!             self.counter += 1;
//!             None
//!         } else {
//!             // Once the counter reaches 3, pop this state off the stack.
//!             // This will return control back to the State A.
//!             Some(Transition::Pop)
//!         }
//!     }
//!
//!     fn render(&mut self, _context: &mut Context) -> RenderResult {}
//! }
//! ```
//! It is important to note that only the top [State] can control the [StateStack], as
//! [State::update()] is the only method which can return a [Transition] to the [StateStack].  This
//! is the only way available to user code for sending commands to the [StateStack].  While this
//! may seem bit restrictive, it is done to keep state changes predictable.  When only the
//! top [State] can change the [StateStack], there is no possibility of random parts of the code
//! changing [State] in unpredictable ways.
//!
//! ## The Context Object
//!
//! The [Context] is a dynamic storage container which holds globally-accessible [Engine] state.
//! In line with Wolf Engine's goal of being as flexible as possible, the [Context] allows new
//! [Subcontext] data to be loaded at run-time allowing for far greater extensibility.
//!
//! [Subcontext] objects most commonly store data for the [Engine], but they may also have bundled
//! functionality.  You'll want to reference the documentation for the [Subcontext] you're working
//! with to see what it's capable of.
//!
//! If you have access to the [Context] object, you can retrieve any currently-loaded [Subcontext]
//! using [Context::borrow()] for immutable access, or [Context::borrow_mut()] for a mutable access:
//!
//! ```
//! # use wolf_engine::*;
//! #
//! # pub struct ExampleContext;
//! #
//! # impl Subcontext for ExampleContext {}
//! #
//! # let mut context = Context::new();
//! # context.add(ExampleContext);
//! #
//! // To borrow immutably, use Context::borrow().
//! let example_context = context.borrow::<ExampleContext>();
//! # drop(example_context);
//!
//! // To borrow mutably, use Context::borrow_mut().
//! let example_context_mut = context.borrow_mut::<ExampleContext>();
//! ```
//!
//! **Note:** Borrow rules are enforced at runtime by a [RwLock](std::sync::RwLock) meaning
//! [Context::borrow_mut()] will block the current thread until there are no other references to
//! the requested [Subcontext]. This has the potential to cause dead-locking if the lock is not
//! released. Refer to [RwLock](std::sync::RwLock) for more details.
//!
//! ### Context Extension Traits
//!
//! A common pattern for Wolf Engine is to use an "Extension Traits" pattern to functionality  
//! directly to the [Context] object.  This pattern works by defining some methods it wants to
//! add to the [Context], then providing an implementation for the [Context].  To use an extension
//! trait, you just import it.
//!
//! A good example of this is the [EngineControls](crate::utils::EngineControls) trait which adds
//! the [EngineControls::quit()](crate::utils::EngineControls::quit()) method.
//!
//! ```
//! # use wolf_engine::*;
//! // First you import the extension trait.
//! use wolf_engine::utils::EngineControls;
//!
//! # // Instancing the Engine because a bare Context won't have an EngineContext loaded by
//! # // default.  Without an EngineContext, context.quit() will panic.
//! # let mut engine = Engine::default();
//! # let context = &mut engine.context;
//! #
//! // Then you can use the methods it provides.
//! context.quit() // Will quit the Engine.
//! ```
//!
//! In most cases you'll be using existing extensions, but this pattern makes it easy to add custom
//! extensions to the [Context].  For example:
//!
//! ```
//! # use wolf_engine::*;
//! #
//! // First you define your extension trait.
//! pub trait GreetingExtension {
//!     fn say_hello(&self, name: String) -> String;
//! }
//!
//! // Then you implement it for the Context.
//! impl GreetingExtension for Context {
//!     fn say_hello(&self, name: String) -> String {
//!         let greeting_context = &self.borrow::<GreetingContext>().unwrap();
//!         format!("{}, {}!", greeting_context.greeting, name)
//!     }
//! }
//!
//! #  pub struct GreetingContext {
//! #      pub greeting: String
//! #  }
//! #  
//! #  impl Subcontext for GreetingContext {}
//! #
//! #  let mut context = Context::new();
//! #  context.add(GreetingContext { greeting: "Hello".to_string() });
//! #
//! // Now you can use your custom extension.
//! let message = context.say_hello("World".to_string());
//! assert_eq!(message, "Hello, World!");
//! ```
//!
//! ## Configuring the Engine With the Engine Builder
//!
//! If you want to configure or customize the [Engine], you can do so using the [EngineBuilder].
//! The [EngineBuilder] is responsible for configuring the [Engine] in a simple and convenient way.
//!
//! ```
//! # use wolf_engine::*;
//! #
//! let engine = EngineBuilder::new()
//!     // customize the engine.
//!     .build()
//!     .unwrap();
//! ```
//!
//! The [EngineBuilder] allows you to:
//!
//! - Configure [Engine] settings.
//! - Load [Plugins](Plugin).
//! - Load [Subcontexts](Subcontext).
//! - Set a custom [Scheduler].
//! - Set a custom [MainLoop].
//!
//! See the [EngineBuilder's](EngineBuilder) documentation for more information.
//!
//! ## Engine Plugins / Extending the Engine
//!
//! [Plugins](Plugin) are a very important part of Wolf Engine, as nearly all functionality,
//! including provided Wolf Engine features, are implemented using the [Plugin] system.  In fact,
//! the core [Engine] provides only the [State] system, the [Context], a [MainLoop], and the
//! [Plugin] system.  Everything else is a [Plugin].  
//!
//! Using [Plugins](Plugin) for allows Wolf Engine to be as flexible as possible, as you
//! can pick and choose only the [Plugins](Plugin) you need for your game.  If something is missing,
//! or if you don't like how Wolf Engine implements a feature, you can simply replace it with a
//! custom [Plugin].
//!
//! Plugins can be added at startup using the [EngineBuilder], and are loaded on
//! [EngineBuilder::build()].
//!
//! ```
//! # use wolf_engine::*;
//! #
//! # pub struct ExamplePlugin;
//! # impl Plugin for ExamplePlugin {
//! #    fn setup(&mut self, engine_builder: EngineBuilder) -> PluginResult { Ok(engine_builder) }
//! # }
//! #
//! # impl ExamplePlugin {
//! #     pub fn new() -> Self { Self }
//! # }
//! #
//! let engine = EngineBuilder::new()
//!     .with_plugin(Box::from(ExamplePlugin::new()))
//!     .build()
//!     .unwrap();
//! ```
//!
//! # Examples
//!
//! For more detailed examples see the
//! [Examples Folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples).

mod core;

pub mod contexts;
pub mod events;
pub mod plugins;
pub mod schedulers;
pub mod utils;

#[cfg(feature = "logging")]
pub mod logging;

pub use crate::core::*;

use log::info;

pub(crate) fn log_startup_information() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    info!("Hello from {} v{} - {}", name, version, homepage);
}

pub(crate) fn log_shutdown() {
    info!("Engine has stopped.  Goodbye.")
}
