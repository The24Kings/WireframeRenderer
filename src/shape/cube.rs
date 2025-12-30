use crate::point3d::Point3D;

pub struct Cube;

impl super::Shape for Cube {
    fn vertices() -> Option<Vec<crate::point3d::Point3D>> {
        Some(vec![
            // Back Face
            Point3D::new(0.25, 0.25, 0.25),
            Point3D::new(-0.25, 0.25, 0.25),
            Point3D::new(-0.25, -0.25, 0.25),
            Point3D::new(0.25, -0.25, 0.25),
            // Front Face
            Point3D::new(0.25, 0.25, -0.25),
            Point3D::new(-0.25, 0.25, -0.25),
            Point3D::new(-0.25, -0.25, -0.25),
            Point3D::new(0.25, -0.25, -0.25),
        ])
    }

    fn indices() -> Option<Vec<Vec<usize>>> {
        Some(vec![
            vec![0, 1, 2, 3], // Back
            vec![4, 5, 6, 7], // Front
            vec![0, 4],       // Top Right
            vec![1, 5],       // Top Left
            vec![2, 6],       // Bottom Left
            vec![3, 7],       // Bottom Right
        ])
    }
}
