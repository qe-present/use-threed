use three_d::*;
use rand::Rng;
use rand_distr::{Normal, Distribution};

fn main() {
    let window = Window::new(WindowSettings {
        title: "模拟布朗运行".to_string(),
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

    // 定义粒子数量
    const PARTICLE_COUNT: usize = 1000;

    // 创建高斯分布
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::rng();

    // 生成粒子
    let mut particles = Vec::new();
    let mut positions = Vec::new(); // 存储粒子位置
    for _ in 0..PARTICLE_COUNT {
        // 使用高斯分布生成随机位置
        let position = vec3(
            normal.sample(&mut rng) * 2.0,
            normal.sample(&mut rng) * 2.0,
            normal.sample(&mut rng) * 2.0,
        );
        positions.push(position);

        // 生成随机颜色
        let color = Srgba::new_opaque(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );

        // 创建粒子（使用小立方体表示）
        let mut particle = Gm::new(
            Mesh::new(&context, &CpuMesh::cube()),
            ColorMaterial::new_opaque(
                &context,
                &CpuMaterial {
                    albedo: color,
                    ..Default::default()
                },
            ),
        );
        particle.set_transformation(Mat4::from_translation(position) * Mat4::from_scale(0.05));
        particles.push(particle);
    }

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        // 更新粒子位置（简单布朗运动模拟）
        for (particle, position) in particles.iter_mut().zip(positions.iter_mut()) {
            // 更新位置
            *position += vec3(
                normal.sample(&mut rng) * 0.1,
                normal.sample(&mut rng) * 0.1,
                normal.sample(&mut rng) * 0.1,
            );
            // 应用新的变换
            particle.set_transformation(Mat4::from_translation(*position) * Mat4::from_scale(0.05));
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.1, 0.1, 0.1, 1.0, 1.0))
            .render(&camera, [&axes as &dyn Object].into_iter().chain(particles.iter().map(|p| p as &dyn Object)), &[]);

        FrameOutput::default()
    });
}
