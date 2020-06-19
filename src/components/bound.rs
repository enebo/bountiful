use amethyst::ecs::{Component, DenseVecStorage};

/// Rectangular boundary for collision detection.
#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Bound {
    pub x: f32,
    pub y: f32,
}

impl Bound {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn max(a: f32, b: f32) -> f32 {
        if a < b {
            b
        } else {
            a
        }
    }

    pub fn min(a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }

    // Positions represent center of bound.
    pub fn intersects(&self, (px, py): (f32, f32), (opx, opy): (f32, f32), ob: &Bound) -> bool {
        let (x, y) = (px - self.x / 2., py - self.y / 2.);
        let (ox, oy) = (opx - ob.x / 2., opy - ob.y / 2.);
        let ((llx, lly), (urx, ury)) = ((x, y),(x + self.x, y + self.y));
        let ((ollx, olly), (ourx, oury)) = ((ox, oy),(ox + ob.x, oy + ob.y));

        //println!("A: ll ({},{}) ur ({},{})", llx, lly, urx, ury);
        //println!("B: ll ({},{}) ur ({},{})", ollx, olly, ourx, oury);

        let xilr = Self::max(llx, ollx);
        let yilr = Self::max(lly, olly);
        let xiur = Self::min(urx, ourx);
        let yiur = Self::min(ury, oury);

        xilr < xiur && yilr < yiur
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Bound;

    #[test]
    fn test_intersects() {
        let bound = Bound::new(2., 2.);
        let other_bound = Bound::new(2., 2.);
        let (px, py) = (1., 1.);
        let (opx, opy) = (2., 1.);
        assert!(bound.intersects((px, py), (opx, opy), &other_bound));
        assert!(bound.intersects((opx, opy), (px, py), &other_bound));
        let (opx, opy) = (1., 2.);
        assert!(bound.intersects((px, py), (opx, opy), &other_bound));
        assert!(bound.intersects((opx, opy), (px, py), &other_bound));
        let (opx, opy) = (3.1, 1.);
        assert!(!bound.intersects((px, py), (opx, opy), &other_bound));
     }
}