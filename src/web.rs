use futures::channel::mpsc::unbounded;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = self)]
    static global: web_sys::DedicatedWorkerGlobalScope;
}

#[wasm_bindgen(start)]
pub fn start() {
    let (send, incoming) = unbounded();
    let closure = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
        send.unbounded_send(e.data().into_serde().unwrap()).unwrap();
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);

    global
        .add_event_listener_with_callback("message", closure.into_js_value().unchecked_ref())
        .unwrap();

    let outgoing = Box::pin(futures::sink::unfold((), |_, msg| {
        global.post_message(&JsValue::from_serde(&msg).unwrap()).unwrap();
        async { Ok(()) }
    }));

    spawn_local(crate::run(incoming, outgoing));
}
