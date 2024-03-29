use crate::prelude::*;
pub fn check_loaded_assets(
    splash_timer: Res<SplashTimer>,
    progress_counter: Option<Res<ProgressCounter>>,
) -> Progress {
    if let Some(progress_counter) = progress_counter {
        let progress = progress_counter.progress();
        let progress_perc: f32 = progress.into();
        let progress_perc = if progress_perc.is_nan() { 0.0 } else { progress_perc };
        let force_continue = progress.done == 0 && progress.total == 0;
        if (progress_perc >= 1.0 && splash_timer.finished()) || (force_continue && splash_timer.finished()) {
            return true.into();
        }
    }
    false.into()
}
