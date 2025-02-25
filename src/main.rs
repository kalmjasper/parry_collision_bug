use kiss3d::nalgebra::Vector;
use parry3d::{
    math::{Isometry, Real, Rotation},
    na::{Quaternion, Translation3, UnitQuaternion, Vector3, Vector4},
    query::NonlinearRigidMotion,
};

mod visualise;
use visualise::OrientedBoundingBox;

fn main() {
    let moving_object = OrientedBoundingBox {
        iso: Isometry::from_parts(
            Translation3::new(0.31406713 / 100.0, -99.970345 / 100.0, 35.0 / 100.0),
            UnitQuaternion::new_unchecked(Quaternion::new(
                -7.1985865e-20,
                -8.1328256e-20,
                0.7082167,
                0.70599514,
            )),
        ),
        half_extents: Vector3::new(235.0 / 100.0, 105.0 / 100.0, 110.0 / 100.0), // 470/2, 210/2, 220/2
    };

    let static_object = OrientedBoundingBox {
        iso: Isometry::from_parts(
            Translation3::new(6079.4917 / 100.0, -849.3108 / 100.0, 853.8756 / 100.0),
            UnitQuaternion::new_unchecked(Quaternion::new(
                0.002524775,
                -0.00026210435,
                -0.0012064485,
                0.99999607,
            )),
        ),
        half_extents: Vector3::new(650.0 / 100.0, 330.0 / 100.0, 275.0 / 100.0), // 1300/2, 660/2, 550/2
    };

    let start_iso = Isometry::new(
        Vector3::new(
            6417.564807596277 / 100.0,
            -118.05831542161856 / 100.0,
            979.5212503942566 / 100.0,
        ),
        Vector3::new(0.0, 0.0, 6.705104721532303),
    );

    let end_iso = Isometry::new(
        Vector3::new(
            7005.823063962664 / 100.0,
            -671.1216184788981 / 100.0,
            1451.0680891785653 / 100.0,
        ),
        Vector3::new(0.0, 0.0, 4.934281625826443),
    );

    let linvel = Vector3::new(
        (end_iso.translation.x - start_iso.translation.x) as Real,
        (end_iso.translation.y - start_iso.translation.y) as Real,
        (end_iso.translation.z - start_iso.translation.z) as Real,
    );

    let angvel = Vector3::new(
        end_iso.rotation.euler_angles().0,
        end_iso.rotation.euler_angles().1,
        end_iso.rotation.euler_angles().2,
    );

    let motion = NonlinearRigidMotion {
        start: start_iso,
        local_center: moving_object.iso.inverse().translation.vector.into(),
        linvel,
        angvel,
    };

    let t = (0..20).map(|t| t as f32 / 20.0);

    let motion_objects = t
        .map(|t| {
            let iso = motion.position_at_time(t as Real);
            OrientedBoundingBox {
                iso: iso * moving_object.iso,
                half_extents: moving_object.half_extents,
            }
        })
        .collect::<Vec<_>>();

    visualise::visualise(
        static_object,
        &vec![OrientedBoundingBox {
            iso: start_iso * moving_object.iso,
            half_extents: moving_object.half_extents,
        }],
    );
}
