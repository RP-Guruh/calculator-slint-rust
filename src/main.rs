// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn print_hell(text: &str) -> &str {
    text
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();
    println!("{}", print_hell("Hello world!"));

    // append angka yang dipilih agar tampil ke layar 
    ui.on_append({
        let ui = ui_weak.clone();
        move |val| {
            if let Some(ui) = ui.upgrade() {
                let mut expr = ui.get_expression();
                expr.push_str(&val);
                ui.set_expression(expr);
            }
        }
    });

    // trigger tombol backsapace
    ui.on_backspace({
        let ui = ui_weak.clone();
        move || {
            if let Some(ui) = ui.upgrade() {
                let mut expr = ui.get_expression().to_string();
                expr.pop();
                ui.set_expression(expr.into());
            }
        }
    });

    ui.on_calculate({
        let ui = ui_weak.clone();
        move || {
            if let Some(ui) = ui.upgrade() {
                let expr = ui.get_expression();
                match evalexpr::eval_int(&expr) {
                    Ok(value) => ui.set_result(slint::SharedString::from(format!("{}", value))),
                    Err(error) => ui.set_result(slint::SharedString::from(format!("Error: {}", error))),
                }
            }
        }
    });

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}
