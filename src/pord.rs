use std::cell::Cell;
use std::rc::{Rc, Weak};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum PordOrCord{
    Pord(POrd),
    Cord(f32,f32),
    Gord(f32,f32) 
}
//Always use svg for POrds
#[derive(Debug, Clone, Default)]
pub struct POrd {
    r: Cell<f32>,
    theta: Cell<f32>,
    anchor: Weak<PordOrCord>,
}

pub trait Cartesian {
    fn rel_xy(&self) -> (f32,f32);
    fn rel_svg_xy(&self) -> (f32,f32) {
        let (x,y) = self.rel_xy();
        (x, -y)
    }
    fn abs_svg_xy(&self, svg_origin:(f32,f32)) -> (f32,f32);
    fn svg_xy_to(&self, other:impl Cartesian) -> (f32,f32) {
        let svg_origin = (0.0,0.0); 
        let (self_x,self_y) = self.abs_svg_xy(svg_origin);
        let (other_x,other_y) = other.abs_svg_xy(svg_origin);
        (other_x - self_x, other_y - self_y)
    }
    fn angle_to(&self, other:&impl Cartesian) -> f32 {
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
    fn dist_to_sq(&self, other:&impl Cartesian) -> f32 {
        let svg_origin = (0.0,0.0);
        let ((lett_x, lett_y), (word_x,word_y)) = (other.abs_svg_xy(svg_origin),self.abs_svg_xy(svg_origin));
        (word_y - lett_y).powi(2) + (word_x - lett_x).powi(2)
    }
}

pub trait Polar {
    fn r(&self) -> f32;
    fn theta(&self) -> f32;
    fn anchor(&self) -> Weak<PordOrCord>;
    fn anchor_abs_svg_xy(&self, svg_origin:(f32,f32)) -> Option<(f32,f32)> {
        let poc = self.anchor().upgrade()?;
        Some(   match poc.as_ref() {
            PordOrCord::Cord(x,y) => {(*x,*y)}
            PordOrCord::Pord(p) => {p.abs_svg_xy(svg_origin)}
            PordOrCord::Gord(x, y) => (svg_origin.0 + x, svg_origin.1 - y),
        })
    }
}

impl Cartesian for PordOrCord {
    fn rel_xy(&self) -> (f32,f32) {
        match &self {
            PordOrCord::Pord(pord) => pord.rel_xy(),
            PordOrCord::Cord(x, y) => (*x,-*y),
            PordOrCord::Gord(x, y) => (*x,*y),
        }
    }
    fn abs_svg_xy(&self, svg_origin:(f32,f32)) -> (f32,f32) {
        match &self {
            PordOrCord::Cord(x,y) => (*x,*y),
            PordOrCord::Gord(x,y) => (x + svg_origin.0,-y + svg_origin.1),
            PordOrCord::Pord(poi) => poi.abs_svg_xy(svg_origin)
        }
    }
}

impl Cartesian for POrd {
    fn rel_xy(&self) -> (f32,f32) {
        let (a,b) = self.theta().sin_cos();
        (self.r() * a, -self.r() * b)
    }
    fn abs_svg_xy(&self, svg_origin:(f32,f32)) -> (f32,f32) {
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
    pub fn new(radius:f32,theta:f32, anchor:Rc<PordOrCord>) -> POrd {
        let r = Cell::new(radius);
        let angle = Cell::new(theta);
        let anchor = Rc::downgrade(&anchor);
        POrd{r, theta:angle, anchor}
    }
    pub fn add_dist(&mut self, added_dist:f32) {
        let dist = self.r.get_mut();
        *dist += added_dist;
    }
    pub fn set_theta(&mut self, new_theta:f32) {
        let theta = self.theta.get_mut();
        *theta = new_theta;
    }
}

impl From<POrd> for PordOrCord {
    fn from(value: POrd) -> Self {
        PordOrCord::Pord(value)
    }
}

impl Polar for POrd {
    fn r(&self) -> f32 {self.r.get()}
    fn theta(&self) -> f32 {self.theta.get()}
    fn anchor(&self) -> Weak<PordOrCord> {
        self.anchor.clone()
    }
}

impl PordOrCord {
    pub fn gal_origin(svg_origin:(f32,f32)) -> Rc<PordOrCord> {
        Rc::new(PordOrCord::Cord(svg_origin.0, svg_origin.1))
    }
    pub fn get_r_mut(&mut self) -> Option<&mut f32> {
        match self {
            PordOrCord::Pord(p) => Some(p.r.get_mut()),
            _ => None
        }
    }
    pub fn get_theta_mut(&mut self) -> Option<&mut f32> {
        match self {
            PordOrCord::Pord(p) => Some(p.theta.get_mut()),
            _ => None
        }
    }
    pub fn get_anchor(&self) -> Option<Weak<PordOrCord>> {
        match self {
            PordOrCord::Pord(pord) => {
                Some(pord.anchor.clone())
            },
            _ => None
        }
    }
}

impl Default for PordOrCord {
    fn default() -> Self {
        PordOrCord::Cord(0.0, 0.0)
    }
}

#[macro_export] 
macro_rules! poc_rc {
    ($dist:expr,$theta:expr,$anchor:expr) => {
        Rc::new(gallifreyan::pord::PordOrCord::Pord(POrd::new($dist, $theta, $anchor.clone())))
    };
}