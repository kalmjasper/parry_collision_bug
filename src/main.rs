use parry3d::{
    math::{Isometry, Point, Real},
    na::{Quaternion, Translation3, UnitQuaternion, Vector3},
    query::{NonlinearRigidMotion, cast_shapes_nonlinear, contact},
    shape::Cuboid,
};

mod visualise;
use visualise::OrientedCuboid;

fn main() {
    /*
       Define motion for moving object
    */
    let moving_object = Cuboid::new(Vector3::new(235.0, 105.0, 110.0));

    let start_iso = Isometry::from_parts(
        Translation3::new(6458.7905, -209.1331, 1014.52124),
        UnitQuaternion::new_unchecked(Quaternion::new(
            -0.542044,
            5.3359954e-20,
            9.459897e-20,
            -0.84035015,
        )),
    );

    let linvel = Vector3::new(588.2583, -553.06335, 471.54688);
    let angvel = Vector3::new(0.0, 0.0, -1.770823);

    let local_center = Point::new(99.97085, -2.2888184e-5, -35.0);

    let motion = NonlinearRigidMotion {
        start: start_iso,
        local_center: local_center,
        linvel,
        angvel,
    };

    /*
       Define static object
    */

    let static_object_iso = Isometry::from_parts(
        Translation3::new(6079.4917, -849.3108, 853.8756),
        UnitQuaternion::new_unchecked(Quaternion::new(
            0.99999607,
            0.002524775,
            -0.00026210435,
            -0.0012064485,
        )),
    );

    let static_object = Cuboid::new(Vector3::new(650.0, 330.0, 275.0));

    let static_object_motion = NonlinearRigidMotion {
        start: static_object_iso,
        local_center: Point::origin(),
        linvel: Vector3::zeros(),
        angvel: Vector3::zeros(),
    };

    println!("motion1: {:?}", static_object_motion);
    println!("motion2: {:?}", motion);

    let collide_result = cast_shapes_nonlinear(
        &motion,
        &moving_object,
        &static_object_motion,
        &static_object,
        0.0,
        1.0,
        true,
    );

    // Should be true
    println!("Collision result: {:?}", collide_result);

    // Evaluate the motion
    let t = (0..20).map(|t| t as f32 / 20.0);
    let motion_objects = t
        .map(|t| {
            let iso = motion.position_at_time(t as Real);
            OrientedCuboid {
                iso: iso,
                cuboid: moving_object,
            }
        })
        .collect::<Vec<_>>();

    for obj in &motion_objects {
        let contact = contact(
            &obj.iso,
            &obj.cuboid,
            &static_object_iso,
            &static_object,
            0.0,
        )
        .unwrap();
        if let Some(c) = contact {
            println!("Contact with dist: {}", c.dist);
        }
    }

    visualise::visualise(
        OrientedCuboid {
            iso: static_object_iso,
            cuboid: static_object,
        },
        &motion_objects,
    );
}
