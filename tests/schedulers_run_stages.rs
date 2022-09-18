use wolf_engine::schedulers::*;

use mockall_double::double;

#[double]
use wolf_engine::StageCallbacks;

fn should_run_stages<U: 'static + UpdateScheduler, R: 'static + RenderScheduler>(
    update_scheduler: U,
    render_scheduler: R,
) {
    let context = Context::new(); 
    let mut stage_callbacks = StageCallbacks::new();
}
