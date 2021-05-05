use futures::prelude::*;
use futures::channel::mpsc::channel;
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
    let (send, incoming) = channel(1);
    let closure = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
        let mut send = send.clone();
        spawn_local(async move {
            let _ = send.send(e.data().into_serde().unwrap());
        });
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);

    global
        .add_event_listener_with_callback("message", closure.into_js_value().unchecked_ref())
        .unwrap();

    let (outgoing, mut recv) = channel(1);
    spawn_local(async move {
        while let Some(msg) = recv.next().await {
            global.post_message(&JsValue::from_serde(&msg).unwrap()).unwrap();
        }
    });

    spawn_local(crate::main(incoming, outgoing));
}
