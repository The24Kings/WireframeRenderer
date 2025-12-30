use crate::point3d::Point3D;

pub mod cube;

pub trait Shape {
    fn vertices() -> Option<Vec<Point3D>> {
        None
    }

    fn indices() -> Option<Vec<Vec<usize>>> {
        None
    }
}
