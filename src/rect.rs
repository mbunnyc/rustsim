use crate::vec2::Vector2;

pub struct Rect {
    pub pos: Vector2,
    pub size: Vector2,
}

impl Rect {
    /// Creates a new [`Rect`].
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Self { pos, size }
    }

    pub fn clamped_to(&self, limit: Rect) -> Rect {
        let mut size_diff = Vector2::new(0.0, 0.0);
        Rect {
            pos: Vector2::new(
                if self.pos.x < limit.pos.x {
                    size_diff.x = self.pos.x + limit.pos.x;
                    limit.pos.x
                } else {
                    self.pos.x
                },
                if self.pos.y < limit.pos.y {
                    size_diff.y = self.pos.y + limit.pos.y;
                    limit.pos.y
                } else {
                    self.pos.y
                },
            ),
            size: Vector2::new(
                if self.size.x < limit.size.x {
                    limit.size.x + size_diff.x
                } else {
                  self.size.x + size_diff.x
                },
                if self.size.y < limit.size.y {
                    limit.size.y + size_diff.y
                } else {
                    self.size.y + size_diff.y
                },
            ),
        }
    }
}
