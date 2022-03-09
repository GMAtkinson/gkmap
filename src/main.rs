use futures::executor;
use rfd::AsyncFileDialog;
use vizia::*;

use wasm_bindgen::prelude::*;

fn main() {
    
    let window_description = WindowDescription::new();

    let app = Application::new(window_description, |cx|{
        Button::new(cx, |cx|{
            //let file = FileDialog::new().pick_file().expect("Failed to select file path");
            // if let Some(file) = FileDialog::new().pick_file() {
            //     //cx.emit(AppEvent::AddFilePaths(files.clone()));
            //     println!("Open File: {:?}", file);
            // }

            cx.spawn(|cx| {
                let task = AsyncFileDialog::new()
                    .pick_file();
                execute(async {
                    let file = task.await;
            
                    if let Some(file) = file {
                        // If you are on native platform you can just get the path
                        #[cfg(not(target_arch = "wasm32"))]
                        println!("{:?}", file.path());
            
                        // If you care about wasm support you just read() the file
                        file.read().await;

                        //console_log!("Done");
                    }
                });
            });
                //let data = file.unwrap().read().await;
        
        }, |cx|{
            Label::new(cx, "Open")
                .width(Pixels(50.0))
                .font("icon")
                .child_space(Stretch(1.0))
        });
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