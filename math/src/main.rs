use crate::vector_3d::Vector3d;
use crate::matrix_3d::Matrix3d;

mod vector_3d;
mod matrix_3d;

fn main() {
    println!("Hello Math!");

    let matrix = Matrix3d::default();
    let identify = Matrix3d::identity();
    let matrix_vector = Matrix3d::new(
        Vector3d { x: 2.0, y: 2.0, z: 2.0 },
        Vector3d { x: 3.0, y: 3.0, z: 3.0 },
        Vector3d { x: 4.0, y: 4.0, z: 4.0 }
    );

    println!("Matrix[0][0] -> {}", matrix[0][0]);

    println!("Matrix[0] -> {:?}", Vector3d::from(matrix[0]));

    println!("Matrix[0] -> {:?}", Vector3d::from(identify[0]));

    println!("Matrix[0] -> {:?}", Vector3d::from(matrix_vector[0]));

    println!("Matrix -> {:?}", matrix_vector);

    let mut add_assign_matrix = matrix_vector;
    add_assign_matrix += 10.0;
    println!("add_assign_matrix -> {:?}", add_assign_matrix);
    add_assign_matrix -= 10.0;
    println!("add_assign_matrix -> {:?}", add_assign_matrix);
    add_assign_matrix *= 2.0;
    println!("add_assign_matrix -> {:?}", add_assign_matrix);

    let add_matrix = matrix_vector + 10.0;
    println!("add_matrix -> {:?}", add_matrix);

    let add_matrix = add_matrix - 5.0;
    println!("add_matrix -> {:?}", add_matrix);

    let add_matrix = add_matrix * 5.0;
    println!("add_matrix -> {:?}", add_matrix);
}
