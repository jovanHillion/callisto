use crate::vector_3d::Vector3d;

mod vector_3d;

fn main() {
    let mut vector1 = Vector3d {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let vector2 = Vector3d {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };

    println!("Vector1 {:?}", vector1);
    println!("Vector2 {:?}", vector2);

    vector1 += vector2;

    println!("Vector1 {:?}", vector1);

    vector1 = &vector1 + vector2;

    println!("Vector1 {:?}", vector1);
}
