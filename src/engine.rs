use crate::browser;
use crate::Closure;
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use futures::channel::oneshot::channel;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlImageElement, CanvasRenderingContext2d};

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;
    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);

    let success_callback = browser::closure_once(move || {
        log!("loaded");
        if let Some(success_tx) = success_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> =
        browser::closure_once(move |err| {
            log!("error on load image!");
            if let Some(error_tx) = error_tx.lock().ok()
                .and_then(|mut opt| opt.take()) {
                    error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err)));
                }
        });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    complete_rx.await??;
    Ok(image)
}

pub trait Game {
    fn update(&mut self);
    fn draw(&self, context: &CanvasRenderingContext2d);
}

pub struct GameLoop;
type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

impl GameLoop {
    pub async fn start(mut game: impl Game + 'static) -> Result<()> {
        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            game.update();
            game.draw(&browser::context().expect("Context should exist"));
            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));
        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}
