use futures::executor;
use rfd::AsyncFileDialog;
use vizia::*;

mod app_data;
use app_data::*;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
fn main() {
    
    let window_description = WindowDescription::new();

    let app = Application::new(window_description, |cx|{

        AppData {
            visible: false,
            text_size: 0.5,
        }.build(cx);
        
        Button::new(cx, |cx| cx.emit(AppEvent::Test), |cx|{
            Label::new(cx, "Press Me")
        }).space(Pixels(10.0));

        VStack::new(cx, |cx|{
            Slider::new(cx, AppData::text_size, Orientation::Horizontal)
                .on_changing(|cx, val| cx.emit(AppEvent::SetTextSize(val)))
                .width(Pixels(200.0))
                .top(Pixels(50.0))
                .bottom(Pixels(50.0));
            Label::new(cx, "This is a web app!").font_size(AppData::text_size.map(|s| 16.0 + s * 40.0));
        })
        .space(Pixels(10.0))
        .visibility(AppData::visible);

        // Button::new(cx, |cx|{

        //     let event_proxy = cx.event_proxy.as_ref().map(|p| p.make_clone()).unwrap();
        //     let task = AsyncFileDialog::new()
        //         .pick_file();
        //     let event = Event::new(AppEvent::Debug("Very Done".to_string())).target(cx.current);
        //     execute(async move {
        //         let file = task.await;
        
        //         if let Some(file) = file {
        //             // If you are on native platform you can just get the path
        //             #[cfg(not(target_arch = "wasm32"))]
        //             println!("{:?}", file.path());
        
        //             // If you care about wasm support you just read() the file
        //             file.read().await;

        //             event_proxy.send(event).expect("Failed to send message");

        //         }
        //     });
        
        // }, |cx|{
        //     Label::new(cx, "Open")
        //         .width(Pixels(50.0))
        //         .font("icon")
        //         .child_space(Stretch(1.0))
        // });
    });

    app.run();
}


use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}