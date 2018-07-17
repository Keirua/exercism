pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let t = Triangle{
            sides:sides
        };
        return if sides[0] > 0 && sides[1] > 0 && sides[2] > 0 {
            if (sides[0] + sides[1]) >= sides[2]
            && (sides[2] + sides[0]) >= sides[1]
            && (sides[2] + sides[1]) >= sides[0] {
                Some(t)
            }
            else {
                None
            }
        }
        else {
            None
        };
    }

    pub fn is_equilateral(&self) -> bool {
        return self.sides[0] == self.sides[1] 
        && self.sides[0] == self.sides[2];
    }

    pub fn is_scalene(&self) -> bool {
        return !self.is_equilateral() && !self.is_isosceles();
    }

    pub fn is_isosceles(&self) -> bool {
        return self.sides[0] == self.sides[1] 
        || self.sides[0] == self.sides[2]
        || self.sides[1] == self.sides[2];
    }
}
