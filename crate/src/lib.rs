#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate binoxxo;

use wasm_bindgen::prelude::*;
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
            let (class, cell_text) = match board.get(col, row) {
                Field::X => ("fixed", "X"),
                Field::O => ("fixed", "O"),
                Field::Empty => ("guess", "_"),
            };
            cell.set_class_name(class);
            cell.set_attribute("data-row", &row.to_string())?;
            cell.set_attribute("data-col", &col.to_string())?;
            cell.set_text_content(Some(cell_text));
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
