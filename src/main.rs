use wry::application::{platform::windows::WindowBuilderExtWindows, window::Icon};

fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();

    let taskbar_icon = load_icon();

    let window = WindowBuilder::new()
        .with_title("Piyush Suthar")
        .with_window_icon(Some(taskbar_icon.clone()))
        .with_taskbar_icon(Some(taskbar_icon))
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(&get_url())?
        .with_document_title_changed_handler(|webview, title| {
            webview.set_title(title.as_str());
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn get_url() -> String {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        let url = &args[1];
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("http://{}", url)
        }
    } else {
        println!("No URL provided, using default: https://piyushsuthar.github.io/");
        "https://piyushsuthar.github.io/".to_string()
    }
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
        let image = image::load_from_memory(include_bytes!("icon.png"))
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
