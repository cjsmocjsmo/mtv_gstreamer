use gst::glib;
use gst::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = gst::init();

    let pipeline = gst::parse::launch(
        r#"
        playbin uri=file:///home/pimedia/testvid.mp4 video-sink="videoscale ! video/x-raw,width=1200,height=800 ! xvimagesink"
    "#,
    )?
    .downcast::<gst::Pipeline>()
    .unwrap();

    let context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&context), false);

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline.bus().unwrap();
    let _ = bus.add_watch({
        let main_loop = main_loop.clone();
        move |_, msg| {
            use gst::MessageView;
            let main_loop = &main_loop;
            match msg.view() {
                MessageView::Eos(..) => main_loop.quit(),
                MessageView::Error(err) => {
                    eprintln!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    main_loop.quit();
                }
                _ => (),
            };
            glib::ControlFlow::Continue
        }
    })
    .expect("Failed to add bus watch");

    main_loop.run();

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}