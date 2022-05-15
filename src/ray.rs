use crate::Vec3::Point3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Default  for Ray {
    fn default() -> Self {
        Self {
            orig: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Ray {
    pub fn new(origin, direction) -> Self{
        Self {origin, direction}
    }

    pub fn origin(&self)-> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, at: f64) -> Point3 {
        self.orig + at * self.dir
    }
}

