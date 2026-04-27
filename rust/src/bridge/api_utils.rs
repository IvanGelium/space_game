use godot::prelude::*;

pub fn safe_run<F, T>(task_name: &str, f: F) -> Option<T>
where
    F: FnOnce() -> Result<T, String>,
{
    // Этот код сработает только в дебаге (cargo build)
    #[cfg(debug_assertions)]
    let start_time = std::time::Instant::now();

    match f() {
        Ok(result) => {
            // Логируем время только в дебаге
            #[cfg(debug_assertions)]
            godot_print!("[WorldApi] '{}' took {:?}", task_name, start_time.elapsed());

            Some(result)
        }
        Err(err) => {
            // Ошибки логируем всегда, даже в релизе (но можно и их скрыть)
            godot_error!("[WorldApi] Error in '{}': {}", task_name, err);
            None
        }
    }
}
