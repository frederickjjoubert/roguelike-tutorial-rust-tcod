

// A rectangle on the the map used for rooms
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x1)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}


// The +1 business here is a bit subtle:
// the A..B notation specifies a range that’s inclusive at the beginning but exclusive at the end.
// For example 1..5 represents numbers 1, 2, 3 and 4 but not 5.
// So to go through all the values between x1 and x2 (including both),
// we’d have to write x1..(x2 + 1).
// But we want to make sure each room is enclosed in a wall, so we want to go from x1 to x2 exclusive.
// To do that, we add 1 to the first coordinate and subtract one from the second,
// ending up with (x1 + 1)..x2. If x1 is 1 and x2 is 5,
// we would put empty tiles at positions 2, 3 and 4 and leave 1 and 5 solid.
