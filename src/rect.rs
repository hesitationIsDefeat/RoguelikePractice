pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub center: (i32, i32),
}

impl Rect {
    pub fn new(x1: i32, y1: i32, w: i32, h: i32) -> Self {
        Self { x1, y1, x2: x1 + w, y2: y1 + h, center: (x1 + w / 2, y1 + h / 2) }
    }
    pub fn intersect(&self, rect: &Rect) -> bool {
        self.x1 <= rect.x2 && self.x2 >= rect.x1 && self.y1 <= rect.y2 && self.y2 >= rect.y1
    }
}