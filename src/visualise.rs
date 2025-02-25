use kiss3d::light::Light;
use kiss3d::window::Window;
use parry3d::math::Isometry;
use parry3d::na::Vector3;

pub struct OrientedBoundingBox {
    pub iso: Isometry<f32>,
    pub half_extents: Vector3<f32>,
}

fn add_cuboid_node(
    window: &mut Window,
    cuboid: &OrientedBoundingBox,
    color: (f32, f32, f32),
) -> kiss3d::scene::SceneNode {
    println!(
        "Size: x={}, y={}, z={}",
        (cuboid.half_extents.x * 2.0) as f32,
        (cuboid.half_extents.y * 2.0) as f32,
        (cuboid.half_extents.z * 2.0) as f32
    );
    let mut cube = window.add_cube(
        cuboid.half_extents.x * 2.0,
        cuboid.half_extents.y * 2.0,
        cuboid.half_extents.z * 2.0,
    );

    cube.set_color(color.0, color.1, color.2);

    println!(
        "Trans: x={}, y={}, z={}",
        cuboid.iso.translation.vector.x,
        cuboid.iso.translation.vector.y,
        cuboid.iso.translation.vector.z
    );
    cube.set_local_translation(kiss3d::nalgebra::Translation3::new(
        cuboid.iso.translation.vector.x,
        cuboid.iso.translation.vector.y,
        cuboid.iso.translation.vector.z,
    ));
    cube.set_local_rotation(kiss3d::nalgebra::UnitQuaternion::from_quaternion(
        kiss3d::nalgebra::Quaternion::new(
            cuboid.iso.rotation.w,
            cuboid.iso.rotation.i,
            cuboid.iso.rotation.j,
            cuboid.iso.rotation.k,
        ),
    ));

    cube
}

pub fn visualise(static_object: OrientedBoundingBox, moving_object: &[OrientedBoundingBox]) {
    let mut window = Window::new("Parry virtualisation");
    window.set_light(Light::StickToCamera);

    add_cuboid_node(&mut window, &static_object, (0.7, 0.7, 0.7));

    for cuboid in moving_object {
        add_cuboid_node(&mut window, cuboid, (0.7, 0.7, 0.7));
    }

    let eye: kiss3d::nalgebra::OPoint<f32, kiss3d::nalgebra::Const<3>> =
        kiss3d::nalgebra::Point3::new(10.0, 0.0, 0.0);
    let at = kiss3d::nalgebra::Point3::new(
        static_object.iso.translation.x,
        static_object.iso.translation.y,
        static_object.iso.translation.z,
    );
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    camera.set_up_axis(kiss3d::nalgebra::Vector3::new(0.0, 0.0, 1.0));

    while window.render_with_camera(&mut camera) {}
}
