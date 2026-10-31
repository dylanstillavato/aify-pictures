//! Aify: las fuentes instaladas en el ordenador, en la web (Local Font Access API, Chrome y Edge).
//!
//! El navegador pide permiso una vez («… quiere ver y usar tus fuentes»). Se pregunta cuando el
//! usuario elige la herramienta Texto, que es cuando le sirve; si ya dio permiso, se cargan al
//! abrir. Las familias entran en los menús de fuentes como las servidas (`served_fonts`), con la
//! ruta `local:<nombre PostScript>`, y cada archivo se lee del sistema solo cuando hace falta.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

use js_sys::{Array, Function, Object, Promise, Reflect, Uint8Array};
use photocraft_ui_egui::served_fonts::ServedFont;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// Prefijo de la «ruta» de una cara local en la lista de archivos de las fuentes servidas.
pub const PREFIJO: &str = "local:";

thread_local! {
    /// Los `FontData` del navegador, por ruta `local:…`, para leer su archivo cuando se pida.
    static CARAS: RefCell<HashMap<String, JsValue>> = RefCell::new(HashMap::new());
}

/// Aún no hay permiso: se pide la próxima vez que el usuario elija una herramienta de texto.
pub static PEDIR_CON_TEXTO: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Permiso {
    /// El navegador no tiene la API (Safari, Firefox).
    NoDisponible,
    /// Aún no se ha preguntado.
    Preguntar,
    Concedido,
    Denegado,
}

fn js(e: JsValue) -> String {
    e.as_string().unwrap_or_else(|| format!("{e:?}"))
}

fn query_local_fonts() -> Option<Function> {
    let w = web_sys::window()?;
    Reflect::get(&w, &"queryLocalFonts".into()).ok()?.dyn_into().ok()
}

/// El permiso «local-fonts» sin preguntar nada.
pub async fn permiso() -> Permiso {
    if query_local_fonts().is_none() {
        return Permiso::NoDisponible;
    }
    let Some(w) = web_sys::window() else { return Permiso::NoDisponible };
    let consulta = async {
        let permisos = Reflect::get(&w.navigator(), &"permissions".into()).ok().filter(|p| !p.is_undefined())?;
        let query: Function = Reflect::get(&permisos, &"query".into()).ok()?.dyn_into().ok()?;
        let arg = Object::new();
        Reflect::set(&arg, &"name".into(), &"local-fonts".into()).ok()?;
        let promesa: Promise = query.call1(&permisos, &arg).ok()?.dyn_into().ok()?;
        let estado = JsFuture::from(promesa).await.ok()?;
        Reflect::get(&estado, &"state".into()).ok()?.as_string()
    };
    match consulta.await.as_deref() {
        Some("granted") => Permiso::Concedido,
        Some("denied") => Permiso::Denegado,
        _ => Permiso::Preguntar,
    }
}

/// Pide las fuentes al navegador (que pregunta si hace falta) y devuelve una entrada por cara,
/// para `served_fonts::add_fonts`. Las caras ocultas de macOS (familia con punto) se saltan.
pub async fn cargar() -> Result<Vec<ServedFont>, String> {
    let f = query_local_fonts().ok_or("este navegador no deja usar las fuentes instaladas")?;
    let w = web_sys::window().ok_or("sin ventana")?;
    let promesa: Promise = f.call0(&w).map_err(js)?.dyn_into().map_err(|_| "queryLocalFonts no devolvió una promesa")?;
    let lista: Array = JsFuture::from(promesa).await.map_err(js)?.dyn_into().map_err(|_| "queryLocalFonts no devolvió una lista")?;
    let mut fuentes = Vec::new();
    CARAS.with(|caras| {
        let mut caras = caras.borrow_mut();
        for cara in lista.iter() {
            let campo = |n: &str| Reflect::get(&cara, &n.into()).ok().and_then(|v| v.as_string()).unwrap_or_default();
            let (familia, postscript) = (campo("family"), campo("postscriptName"));
            if familia.is_empty() || familia.starts_with('.') || postscript.is_empty() {
                continue;
            }
            let ruta = format!("{PREFIJO}{postscript}");
            if caras.insert(ruta.clone(), cara).is_none() {
                fuentes.push(ServedFont { family: familia, file: ruta, scripts: Vec::new(), integrity: None });
            }
        }
    });
    Ok(fuentes)
}

/// Lee el archivo de una cara local (`local:<PostScript>`).
pub async fn leer(ruta: &str) -> Result<Vec<u8>, String> {
    let cara = CARAS.with(|c| c.borrow().get(ruta).cloned()).ok_or_else(|| format!("no hay ninguna fuente {ruta}"))?;
    let blob_fn: Function = Reflect::get(&cara, &"blob".into()).map_err(js)?.dyn_into().map_err(|_| "FontData sin blob()")?;
    let promesa: Promise = blob_fn.call0(&cara).map_err(js)?.dyn_into().map_err(|_| "blob() no devolvió una promesa")?;
    let blob: web_sys::Blob = JsFuture::from(promesa).await.map_err(js)?.dyn_into().map_err(|_| "no es un Blob")?;
    let datos = JsFuture::from(blob.array_buffer()).await.map_err(js)?;
    Ok(Uint8Array::new(&datos).to_vec())
}
