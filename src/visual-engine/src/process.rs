use super::utils;

use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassContents,
    },
    swapchain::{
        acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

impl super::VisualEngine {
    pub fn start(mut self) {
        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(self.device.clone(), Default::default());

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(self.device.clone()).boxed());
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swapchain = true;
                }
                Event::RedrawEventsCleared => {
                    let window = self
                        .surface
                        .object()
                        .unwrap()
                        .downcast_ref::<Window>()
                        .unwrap();
                    let dimensions = window.inner_size();
                    if dimensions.width == 0 || dimensions.height == 0 {
                        return;
                    }

                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    if recreate_swapchain {
                        let (new_swapchain, new_images) =
                            match self.swapchain.recreate(SwapchainCreateInfo {
                                image_extent: dimensions.into(),
                                ..self.swapchain.create_info()
                            }) {
                                Ok(r) => r,
                                Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => {
                                    return
                                }
                                Err(e) => panic!("Failed to recreate swapchain: {e:?}."),
                            };

                        self.swapchain = new_swapchain;
                        self.framebuffers = utils::window_size_dependent_setup(
                            &new_images,
                            self.render_passes.normal.clone(),
                            &mut self.viewport,
                        );
                        recreate_swapchain = false;
                    }

                    let (image_index, suboptimal, acquire_future) =
                        match acquire_next_image(self.swapchain.clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {e:?}."),
                        };

                    if suboptimal {
                        recreate_swapchain = true;
                    };

                    let mut builder = AutoCommandBufferBuilder::primary(
                        &command_buffer_allocator,
                        self.queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    )
                    .unwrap();

                    builder
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![Some([0.0, 0.0, 0.0, 0.0].into())],
                                ..RenderPassBeginInfo::framebuffer(
                                    self.framebuffers[image_index as usize].clone(),
                                )
                            },
                            SubpassContents::Inline,
                        )
                        .unwrap()
                        .set_viewport(0, [self.viewport.clone()])
                        .bind_pipeline_graphics(self.graphics_pipeline.clone());
                    self.loaded_meshes
                        .as_mut_slice()
                        .into_iter()
                        .for_each(|mesh| {
                            builder
                                .bind_vertex_buffers(
                                    0,
                                    mesh.mesh_buffer.as_ref().unwrap().vertex_buffer.clone(),
                                )
                                .draw(
                                    mesh.mesh_buffer.as_ref().unwrap().vertex_buffer.len() as u32,
                                    1,
                                    0,
                                    0,
                                )
                                .unwrap();
                        });
                    builder.end_render_pass().unwrap();

                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(
                            self.queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(
                                self.swapchain.clone(),
                                image_index,
                            ),
                        )
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                        Err(e) => {
                            panic!("Failed to flush future: {e:?}.");
                        }
                    }
                }

                _ => (),
            });
    }
}
