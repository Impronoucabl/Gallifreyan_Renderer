use std::rc::{Rc, Weak};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub enum PordOrCord{
    Pord(POrd),
    Cord(f64,f64),
    Gord(f64,f64)
}
//Always use svg for POrds
#[derive(Debug, Clone)]
pub struct POrd {
    r: Rc<f64>,
    theta: Rc<f64>,
    anchor: Weak<PordOrCord>,
}

pub trait Cartesian {
    fn rel_xy(&self) -> (f64,f64);
    fn rel_svg_xy(&self) -> (f64,f64) {
        let (x,y) = self.rel_xy();
        (x, -y)
    }
    fn abs_svg_xy(&self, svg_origin:(f64,f64)) -> (f64,f64);
    fn angle_to(&self, other:&impl Cartesian) -> f64 {
        //we don't actually care about the final translation
        let svg_origin = (0.0,0.0); 
        let (x1,y1) = self.abs_svg_xy(svg_origin);
        let (x2,y2) = other.abs_svg_xy(svg_origin);
        let raw = (x2-x1).atan2(y2-y1);
        if raw < 0.0 {
            raw + 2.0*PI
        } else {
            raw
        }
    }
    fn dist_to_sq(&self, other:&impl Cartesian) -> f64 {
        let svg_origin = (0.0,0.0);
        let ((lett_x, lett_y), (word_x,word_y)) = (other.abs_svg_xy(svg_origin),self.abs_svg_xy(svg_origin));
        (word_y - lett_y).powi(2) + (word_x - lett_x).powi(2)
    }
}

impl Cartesian for PordOrCord {
    fn rel_xy(&self) -> (f64,f64) {
        match &self {
            PordOrCord::Pord(pord) => pord.rel_xy(),
            PordOrCord::Cord(x, y) => (*x,-*y),
            PordOrCord::Gord(x, y) => (*x,*y),
        }
    }
    fn abs_svg_xy(&self, svg_origin:(f64,f64)) -> (f64,f64) {
        match &self {
            PordOrCord::Cord(x,y) => (*x,*y),
            PordOrCord::Gord(x,y) => (x + svg_origin.0,-y + svg_origin.1),
            PordOrCord::Pord(poi) => poi.abs_svg_xy(svg_origin)
        }
    }
}

impl Cartesian for POrd {
    fn rel_xy(&self) -> (f64,f64) {
        let (a,b) = self.theta().sin_cos();
        (*self.r() * a, -*self.r() * b)
    }
    fn abs_svg_xy(&self, svg_origin:(f64,f64)) -> (f64,f64) {
        let (x,y) = match self.anchor_abs_svg_xy(svg_origin) {
            Some(anchor_xy) => anchor_xy,
            None => {
                println!("Anchor missing. Using svg origin instead.");
                svg_origin
            },
        };
        let (x_rel, y_rel) = self.rel_svg_xy();
        (x+x_rel,y+y_rel)
    } 
}

impl POrd {
    pub fn new(radius:f64,theta:f64, anchor:&Rc<PordOrCord>) -> POrd {
        let r = Rc::new(radius);
        let angle = Rc::new(theta);
        let anchor = Rc::downgrade(anchor);
        POrd{r, theta:angle, anchor}
    }
    pub fn add_dist(&mut self, added_dist:f64) {
        let dist = Rc::get_mut(&mut self.r).expect("someoneelse using this");
        *dist += added_dist;
    }
    pub fn set_theta(&mut self, new_theta:f64) {
        let theta = Rc::get_mut(&mut self.theta).expect("SomeoneElse using this");
        *theta = new_theta;
    }
}

impl Polar for POrd {
    fn r(&self) -> Rc<f64> {self.r.clone()}
    fn theta(&self) -> Rc<f64> {self.theta.clone()}
    fn anchor(&self) -> Weak<PordOrCord> {
        self.anchor.clone()
    }
}

pub trait Polar {
    fn r(&self) -> Rc<f64>;
    fn theta(&self) -> Rc<f64>;
    fn anchor(&self) -> Weak<PordOrCord>;
    fn anchor_abs_svg_xy(&self, svg_origin:(f64,f64)) -> Option<(f64,f64)> {
        let poc = self.anchor().upgrade()?;
        Some(   match poc.as_ref() {
            PordOrCord::Cord(x,y) => {(*x,*y)}
            PordOrCord::Pord(p) => {p.abs_svg_xy(svg_origin)}
            PordOrCord::Gord(x, y) => (svg_origin.0 + x, svg_origin.1 - y),
        })
    }
}