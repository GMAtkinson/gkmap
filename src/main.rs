use vizia::*;


fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |cx|{
        Label::new(cx, "test");
    });

    app.run();
}
