slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.on_thanks(|| {
        if let Ok(popup) = PopUp::new() {
            popup.run().ok();

            popup.on_ok(|| {
                slint::quit_event_loop().ok();
            });
        }
    });

    main_window.run()
}
