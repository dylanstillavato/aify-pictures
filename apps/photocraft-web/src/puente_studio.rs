//! Aify: el puente con la pizarra de @Studio. Van en el mismo origen (studio.useaify.app/ y
//! /pictures/), así que se hablan por `BroadcastChannel('aify-pictures')`:
//!
//!   Studio → editor  { tipo: 'abrir-imagen', id, nombre, blob }   abre la imagen como documento
//!   editor → Studio  { tipo: 'imagen-recibida', id, nombre }      (Studio deja de reenviarla)
//!   Studio → editor  { tipo: 'pedir-imagen', id }                 «tráela de vuelta»
//!   editor → Studio  { tipo: 'imagen-editada', id, nombre, blob } el documento activo, en PNG
//!                    { tipo: 'imagen-editada', id, error }        si no hay ninguno
//!   editor → Studio  { tipo: 'editor-listo' }                     al arrancar
//!
//! Studio reenvía «abrir-imagen» cada segundo hasta que llega «imagen-recibida»: el `id` evita
//! abrirla dos veces.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use js_sys::{Array, Object, Reflect, Uint8Array};
use photocraft_ui_egui::PhotocraftApp;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

const CANAL: &str = "aify-pictures";

thread_local! {
    static CANAL_JS: RefCell<Option<web_sys::BroadcastChannel>> = const { RefCell::new(None) };
    static VISTOS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static PEDIDOS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn enviar(datos: &Object) {
    CANAL_JS.with(|c| {
        if let Some(c) = c.borrow().as_ref() {
            let _ = c.post_message(datos);
        }
    });
}

fn objeto(pares: &[(&str, JsValue)]) -> Object {
    let o = Object::new();
    for (k, v) in pares {
        let _ = Reflect::set(&o, &(*k).into(), v);
    }
    o
}

fn campo(v: &JsValue, k: &str) -> Option<String> {
    Reflect::get(v, &k.into()).ok()?.as_string()
}

/// Empieza a escuchar a la pizarra. `recibir(nombre, bytes)` abre la imagen (la cola de archivos
/// del editor) y `despertar` pide un fotograma, para que se atienda cuanto antes.
pub fn iniciar(recibir: impl Fn(String, Vec<u8>) + 'static, despertar: impl Fn() + 'static) {
    let Ok(canal) = web_sys::BroadcastChannel::new(CANAL) else { return };
    let recibir = Rc::new(recibir);
    let al_mensaje = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
        let datos = e.data();
        match campo(&datos, "tipo").as_deref() {
            Some("abrir-imagen") => {
                let nombre = campo(&datos, "nombre").unwrap_or_else(|| "imagen.png".into());
                let id = campo(&datos, "id").unwrap_or_else(|| nombre.clone());
                let recibida = objeto(&[("tipo", "imagen-recibida".into()), ("nombre", nombre.clone().into()), ("id", id.clone().into())]);
                if !VISTOS.with(|v| v.borrow_mut().insert(id)) {
                    enviar(&recibida);
                    return;
                }
                let Some(blob) = Reflect::get(&datos, &"blob".into()).ok().and_then(|b| b.dyn_into::<web_sys::Blob>().ok()) else { return };
                let recibir = recibir.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await {
                        Ok(buf) => {
                            recibir(nombre, Uint8Array::new(&buf).to_vec());
                            enviar(&recibida);
                        }
                        Err(e) => log::error!("@Studio: no se pudo leer la imagen: {e:?}"),
                    }
                });
            }
            Some("pedir-imagen") => {
                PEDIDOS.with(|p| p.borrow_mut().push(campo(&datos, "id").unwrap_or_default()));
                despertar();
            }
            _ => {}
        }
    });
    canal.set_onmessage(Some(al_mensaje.as_ref().unchecked_ref()));
    al_mensaje.forget();
    CANAL_JS.with(|c| *c.borrow_mut() = Some(canal));
    enviar(&objeto(&[("tipo", "editor-listo".into())]));
}

/// Atiende los «pedir-imagen»: manda el documento activo, aplanado, en PNG. Llamar cada fotograma.
pub fn atender(app: &PhotocraftApp) {
    let pedidos: Vec<String> = PEDIDOS.with(|p| std::mem::take(&mut *p.borrow_mut()));
    for id in pedidos {
        let error = |texto: &str| objeto(&[("tipo", "imagen-editada".into()), ("id", id.clone().into()), ("error", texto.into())]);
        let respuesta = match app.session.active() {
            None => error("No hay ninguna imagen abierta en @Pictures."),
            Some(st) => match photocraft_io::export(&st.doc, "imagen.png", &photocraft_io::ExportOptions::default()) {
                Err(e) => error(&format!("No se pudo exportar la imagen: {e}")),
                Ok(r) => {
                    let partes = Array::of1(&Uint8Array::from(&r.bytes[..]));
                    let opts = web_sys::BlobPropertyBag::new();
                    opts.set_type("image/png");
                    match web_sys::Blob::new_with_u8_array_sequence_and_options(&partes, &opts) {
                        Ok(blob) => objeto(&[
                            ("tipo", "imagen-editada".into()),
                            ("id", id.clone().into()),
                            ("nombre", format!("{}.png", st.doc.name).into()),
                            ("blob", blob.into()),
                        ]),
                        Err(_) => error("No se pudo preparar la imagen."),
                    }
                }
            },
        };
        enviar(&respuesta);
    }
}
