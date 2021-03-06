use imui_glium::*;

#[derive(Debug)]
struct Interface {
    num_buttons: u32,
    draggables: Vec<u32>,
}

impl Interface {
    fn update(&mut self, glue: &mut Glue) {
        loop {
            let mut updated = false;

            glue.update(|ui| {
                ui.vbox("container", |ui| {
                    ui.text("hello", "Hello, world!").center_x();

                    ui.hbox("buttons", |ui| {
                        for i in 0..self.num_buttons {
                            if ui.button(i, format!("Button {}", i)).clicked() {
                                println!("button {} clicked", i);

                                self.num_buttons += 1;

                                // If state changes during an update, its recommended that you update again afterward so
                                // you always show the latest data.
                                updated = true;
                            }
                        }
                    });

                    ui.text("num buttons", format!("Above are {} buttons", self.num_buttons));

                    if ui.vdraglist("draglist", &mut self.draggables, |ui, item| {
                        ui.pad(-1, 10.0);
                        ui.text(0, "Drag me!").center_y();
                        ui.pad(1, 10.0);
                        ui.button(2, format!("{}", item)).with_width(36.0);
                    }) {
                        updated = true;
                    }
                });
            });

            if !updated {
                break;
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let wb = imui_glium::glium::glutin::window::WindowBuilder::new()
        .with_title("imui_glium")
        .with_inner_size(imui_glium::glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_visible(false); // Font loading can take a while, so we'll only show the window once loading is done.
    let cb = imui_glium::glium::glutin::ContextBuilder::new()
        .with_multisampling(4)
        .with_srgb(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut glue = Glue::new(&display).unwrap();

    glue.atlas().insert("button", "assets/tex/button.png").unwrap();
    glue.atlas().insert("button_pressed", "assets/tex/button_pressed.png").unwrap();
    glue.atlas().insert("window", "assets/tex/window.png").unwrap();
    glue.atlas().insert("white", "assets/tex/white.png").unwrap();
    glue.load_font(include_bytes!("../../assets/Inter-Medium.otf")).unwrap();

    let mut interface = Interface {
        num_buttons: 1,
        draggables: vec![1, 2, 3, 4, 5],
    };
    interface.update(&mut glue);

    {
        let mut surface = display.draw();
        surface.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), -1000.0);
        glue.draw(&mut surface, &display).unwrap();
        surface.finish().unwrap();
    }

    // Loading is done and we've drawn something; show the window.
    display.gl_window().window().set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let mut redraw = false;

        match event {
            Event::WindowEvent { event, window_id: _ } => {
                if glue.handle_window_event(&event, &display) {
                    // Some input happened, update the interface.
                    interface.update(&mut glue);
                }

                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_window_id) => {
                redraw = true;
            }
            _ => {}
        }

        if glue.needs_redraw() || redraw {
            let mut surface = display.draw();
            surface.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), -1000.0);
            glue.draw(&mut surface, &display).unwrap();
            surface.finish().unwrap();
        }
    })
}
