use vizia::*;


fn main() {
    let window_description = WindowDescription::new().with_canvas("win");
    let app = Application::new(window_description, |cx|{
        Label::new(cx, "test");
    });

    app.run();
}
