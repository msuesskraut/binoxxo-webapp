#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;
extern crate binoxxo;

use std::sync::RwLock;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use binoxxo::field::{Board, Field};
use binoxxo::rules::{is_board_valid, is_board_full};

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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

const BOARD_SIZE : usize = 6;
const BINOXXO_LEVEL : usize = 5;

fn field_to_str(field: Field) -> &'static str {
    match field {
        Field::X => "X",
        Field::O => "O",
        Field::Empty => "_",
    }
}

fn next_field_value(cur_field: Field) -> Field {
    match cur_field {
        Field::Empty => Field::X,
        Field::X => Field::O,
        Field::O => Field::Empty,
    }
}

fn change_field(cell: &web_sys::Element, board: &mut Board, row: usize, col: usize) {
    let field = board.get(col, row);
    let next_field = next_field_value(field);
    if Field::Empty == next_field {
        board.clear(col, row);
    } else {
        board.set(col, row, next_field);
    }
    cell.set_text_content(Some(field_to_str(next_field)));
}

fn handle_guess(cell: &web_sys::Element, board: &mut Board, row: usize, col: usize) {
    web_sys::console::log_5(&"handle_guess(..., row =".into(), &(row as f64).into(), &", col =".into(), &(col as f64).into(), &")".into());
    change_field(cell, board, row, col);
    if is_board_full(board) {
        let msg = if is_board_valid(board) {
            "You have solved the puzzle."
        } else {
            "No, that is not right. Please continue solving."
        };
        alert(msg);
    }
}

fn board_to_html(board: &Board, doc: &web_sys::Document) -> Result<web_sys::Element, JsValue> {
    let table = doc.create_element("table")?.dyn_into::<web_sys::HtmlTableElement>()?;
    let board_size = board.get_size();

    let the_board: Arc<RwLock<Board>> = Arc::new(RwLock::new((*board).clone()));

    for row in 0..board_size {
        let table_row = table.insert_row()?.dyn_into::<web_sys::HtmlTableRowElement>()?;
        for col in 0..board_size {
            let cell = table_row.insert_cell()?;
            let field = board.get(col, row);
            let (class, need_callback) = match field {
                Field::X => ("fixed", false),
                Field::O => ("fixed", false),
                Field::Empty => ("guess", true),
            };
            cell.set_class_name(class);
            cell.set_text_content(Some(field_to_str(field)));
            if need_callback {
                let board = the_board.clone();
                let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    let mut board = board.write().unwrap();
                    let cell = event.target().unwrap().dyn_into::<web_sys::Element>().ok().unwrap();
                    handle_guess(&cell, &mut board, row, col);
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
