use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

impl super::VisualEngine {
    pub fn process(mut self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::NewEvents(_) => (),
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => self.visual_swapchain.recreate(),
                    _ => (),
                },
                Event::DeviceEvent { device_id, event } => (),
                Event::UserEvent(_) => (),
                Event::Suspended => (),
                Event::Resumed => (),
                Event::MainEventsCleared => (),
                Event::RedrawRequested(_) => (),
                Event::RedrawEventsCleared => (),
                Event::LoopDestroyed => (),
            });
    }
}
