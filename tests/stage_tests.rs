use wolf_engine::*; 
use wolf_engine::schedulers::*;
use wolf_engine::events::*;

use test_case::test_case;
use ntest::timeout;


#[test_case(FixedUpdateScheduler::default(), SimpleRenderScheduler)]
fn should_run_all_stages<U: UpdateScheduler, R: RenderScheduler>(update_scheduler: U, render_scheduler: R) {
    let events = EventQueue::<Stage>::new();
    let mut engine = Engine::default();
    engine.context.add(events).unwrap();

    engine.update();
    engine.render();
    
    let events = engine.context.flush_events::<Stage>();
    assert!(events.contains(&Stage::PreUpdate)); 
}
