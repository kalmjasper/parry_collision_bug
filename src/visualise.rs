use kiss3d::light::Light;
use kiss3d::window::Window;
use parry3d::math::Isometry;
use parry3d::shape::Cuboid;

// Kiss3d does not work well with large cuboids, so we scale down the whole scene
const SCALE_FACTOR: f32 = 1.0 / 100.0;

pub struct OrientedCuboid {
    pub iso: Isometry<f32>,
    pub cuboid: Cuboid,
}

fn add_cuboid_node(
    window: &mut Window,
    oriented_cuboid: &OrientedCuboid,
    color: (f32, f32, f32),
) -> kiss3d::scene::SceneNode {
    // Need to convert between parry3d nalgebra and kiss3d nalgebra
    let mut cube = window.add_cube(
        oriented_cuboid.cuboid.half_extents.x * 2.0 * SCALE_FACTOR,
        oriented_cuboid.cuboid.half_extents.y * 2.0 * SCALE_FACTOR,
        oriented_cuboid.cuboid.half_extents.z * 2.0 * SCALE_FACTOR,
    );

    cube.set_color(color.0, color.1, color.2);

    cube.set_local_translation(kiss3d::nalgebra::Translation3::new(
        oriented_cuboid.iso.translation.vector.x * SCALE_FACTOR,
        oriented_cuboid.iso.translation.vector.y * SCALE_FACTOR,
        oriented_cuboid.iso.translation.vector.z * SCALE_FACTOR,
    ));
    cube.set_local_rotation(kiss3d::nalgebra::UnitQuaternion::from_quaternion(
        kiss3d::nalgebra::Quaternion::new(
            oriented_cuboid.iso.rotation.w,
            oriented_cuboid.iso.rotation.i,
            oriented_cuboid.iso.rotation.j,
            oriented_cuboid.iso.rotation.k,
        ),
    ));

    cube
}

pub fn visualise(static_object: OrientedCuboid, moving_object: &[OrientedCuboid]) {
    let mut window = Window::new("Parry virtualisation");
    window.set_light(Light::StickToCamera);

    add_cuboid_node(&mut window, &static_object, (0.7, 0.7, 0.7));

    for cuboid in moving_object {
        add_cuboid_node(&mut window, cuboid, (1.0, 0.0, 0.0));
    }

    let eye: kiss3d::nalgebra::OPoint<f32, kiss3d::nalgebra::Const<3>> =
        kiss3d::nalgebra::Point3::new(100.0, 0.0, 0.0);
    let at = kiss3d::nalgebra::Point3::new(
        static_object.iso.translation.x * SCALE_FACTOR,
        static_object.iso.translation.y * SCALE_FACTOR,
        static_object.iso.translation.z * SCALE_FACTOR,
    );
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    camera.set_up_axis(kiss3d::nalgebra::Vector3::new(0.0, 0.0, 1.0));

    while window.render_with_camera(&mut camera) {}
}
