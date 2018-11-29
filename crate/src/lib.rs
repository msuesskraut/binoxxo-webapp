#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate binoxxo;

use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use binoxxo::field::{Board, Field};
use wasm_bindgen::JsCast;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

const BOARD_SIZE : usize = 10;
const BINOXXO_LEVEL : usize = 10;

fn board_to_html(board: &Board, doc: &web_sys::Document) -> Result<web_sys::Element, JsValue> {
    let table = doc.create_element("table")?.dyn_into::<web_sys::HtmlTableElement>()?;
    let board_size = board.get_size();

    for row in 0..board_size {
        let table_row = table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;
        for col in 0..board_size {
            let cell = table_row.insert_cell()?;
            let (class, cell_text, need_callback) = match board.get(col, row) {
                Field::X => ("fixed", "X", false),
                Field::O => ("fixed", "O", false),
                Field::Empty => ("guess", "_", true),
            };
            cell.set_class_name(class);
            cell.set_text_content(Some(cell_text));
            if need_callback {
                let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    web_sys::console::log_4(
                        &"row =".into(), &(row as f64).into(),
                        &"col =".into(), &(col as f64).into());
                    let cell = event.target().unwrap().dyn_into::<web_sys::Element>().ok().unwrap();
                    cell.set_text_content(Some("M"));
                }) as Box<FnMut(web_sys::Event)>);
                cell.add_event_listener_with_event_listener("click", cb.as_ref().unchecked_ref())?;
                cb.forget();
            }
        }
    }

    Ok(table.into())
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let board = binoxxo::bruteforce::create_puzzle_board(BOARD_SIZE, BINOXXO_LEVEL);
    let html_board = board_to_html(&board, &document)?;

    let body = document.body().expect("should have a body");
    let board_elem: web_sys::Element = body.query_selector("#board")?
        .ok_or(JsValue::from_str("div#board not found"))?;
    board_elem.append_child(&html_board)?;

    Ok(())
}
