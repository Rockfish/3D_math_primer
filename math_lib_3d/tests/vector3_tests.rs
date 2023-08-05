use math_lib_3d;
use math_lib_3d::vector3::*;

#[test]
fn test_mul_vector_by_scalar() {
    let v1 = Vector3::identity();
    let r1 = &v1 * 1.0;
    assert_eq!(&v1, &r1);

    let v2 = Vector3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };
    let r2 = &v2 * 2.0;
    let expect2 = Vector3 {
        x: 4.0,
        y: 6.0,
        z: 8.0,
    };
    assert_eq!(&r2, &expect2);
}

#[test]
fn test_mul_scalar_by_vector() {
    let v1 = Vector3::identity();
    let r1 = 1.0 * &v1;
    assert_eq!(&v1, &r1);

    let v2 = Vector3 {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };
    let r2 = 2.0 * &v2;
    let expect2 = Vector3 {
        x: 4.0,
        y: 6.0,
        z: 8.0,
    };
    assert_eq!(&r2, &expect2);
}
