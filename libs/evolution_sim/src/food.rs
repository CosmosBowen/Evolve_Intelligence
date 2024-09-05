use nalgebra as na;

pub struct Food {
    pub(crate) position: na::Point2<f32>,
    pub(crate) is_eaten: bool,
}

impl Food {
    pub fn new(position: na::Point2<f32>) -> Self {
        Self { position, is_eaten: false }
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}