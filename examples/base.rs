use three_d::*;

fn main() {
    let window = Window::new(WindowSettings {
        title: "use-threed".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    })
        .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 5.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );
    let axes = Axes::new(&context, 0.1, 4.0);

    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        ColorMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::BLUE,
                ..Default::default()
            },
        ),
    );



    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        // 旋转立方体
        cube.set_transformation(Mat4::from_angle_y(radians(
            (frame_input.accumulated_time * 0.001) as f32,
        )));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0))
            .render(&camera, axes.into_iter().chain(&cube) ,&[]);

        FrameOutput::default()
    });
}
