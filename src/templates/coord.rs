pub const COORD: &'static str = r#"

pub trait SpatialHashTableCoord: Copy {
    fn x(self) -> usize;
    fn y(self) -> usize;
    fn to_linear_index(self, width: usize) -> usize {
        self.y() * width + self.x()
    }
}

impl SpatialHashTableCoord for (usize, usize) {
    fn x(self) -> usize { self.0 }
    fn y(self) -> usize { self.1 }
}
"#;
