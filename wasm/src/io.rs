#[cfg(target_arch = "wasm32")]
mod io {
    use crate::error::AppError;
    use futures::StreamExt;
    use futures::channel::mpsc;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::*;
    use web_sys::{CustomEvent, Event, EventTarget, WorkerGlobalScope, js_sys};

    pub struct InputWaiter {
        rx: mpsc::UnboundedReceiver<String>,
    }
    impl InputWaiter {
        pub fn new() -> InputWaiter {
            let global = js_sys::global();
            let worker_scope: WorkerGlobalScope = global.unchecked_into();
            let target: EventTarget = worker_scope.into();

            InputWaiter {
                rx: Self::wait_input_event(&target, "input-event"),
            }
        }

        pub async fn next(&mut self) -> Result<String, AppError> {
            match self.rx.next().await {
                Some(s) => Ok(s),
                None => Err(AppError::input_error("Failed to get input")),
            }
        }
        fn wait_input_event(
            target: &EventTarget,
            event_name: &str,
        ) -> mpsc::UnboundedReceiver<String> {
            let (tx, rx) = mpsc::unbounded::<String>();
            let closure = Closure::wrap(Box::new(move |ev: Event| {
                // Try to downcast to CustomEvent
                if let Ok(custom_event) = ev.dyn_into::<CustomEvent>() {
                    // Convert JsValue to String if possible
                    if let Some(s) = custom_event.detail().as_string() {
                        let _ = tx.unbounded_send(s);
                    }
                }
            })
                as Box<dyn FnMut(Event)>);

            target
                .add_event_listener_with_callback(
                    event_name,
                    closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            closure.forget();

            rx
        }
    }
}
