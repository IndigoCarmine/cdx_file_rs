use crate::cdx::values::*;

const POINT_TO_PIXEL_SCALE: f64 = 1.0 / 65536.0;    

impl Point2d {
    pub fn to_backend_point(&self) -> crate::cdx::values::Point2d {
        Point2d {
            x: self.x as f64 * POINT_TO_PIXEL_SCALE,
            y: self.y as f64 * POINT_TO_PIXEL_SCALE,
        }
    }
}

impl Point3d {
    pub fn to_backend_point(&self) -> crate::cdx::values::Point3d {
        Point3d {
            x: self.x as f64 * POINT_TO_PIXEL_SCALE,
            y: self.y as f64 * POINT_TO_PIXEL_SCALE,
            z: self.z as f64 * POINT_TO_PIXEL_SCALE,
        }
    }
}

impl Rectangle {
    pub fn to_backend_rect(&self) -> crate::cdx::values::Rectangle {
        Rectangle {
            left: self.left as f64 * POINT_TO_PIXEL_SCALE,
            top: self.top as f64 * POINT_TO_PIXEL_SCALE,
            right: self.right as f64 * POINT_TO_PIXEL_SCALE,
            bottom: self.bottom as f64 * POINT_TO_PIXEL_SCALE,
        }
    }
}

pub trait ToBackendF32 {
    fn to_backend_f32(&self) -> f32;
}

impl ToBackendF32 for f32 {
    fn to_backend_f32(&self) -> f32 {
        (*self as f64 * POINT_TO_PIXEL_SCALE) as f32
    }
}