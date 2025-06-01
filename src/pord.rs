use std::rc::{Rc, Weak};

#[derive(Clone)]
pub enum PordOrCord{
    Pord(POrd),
    Cord(f64,f64),
}

#[derive(Clone)]
pub struct POrd {
    r: Rc<f64>,
    theta: Rc<f64>,
    anchor: Weak<PordOrCord>,
}

impl POrd {
    pub fn new(radius:f64,theta:f64, anchor:&Rc<PordOrCord>) -> POrd {
        let r = Rc::new(radius);
        let angle = Rc::new(theta);
        let anchor = Rc::downgrade(anchor);
        POrd{r, theta:angle, anchor}
    }
}

impl Cartesian for PordOrCord {
    fn xy(&self) -> (f64,f64) {
        match &self {
            PordOrCord::Pord(pord) => pord.xy(),
            PordOrCord::Cord(x, y) => (*x,*y),
        }
    }
}

impl PordOrCord {
    pub fn svg_xy(&self, svg_origin:(f64,f64)) -> (f64,f64) {
        match &self {
            PordOrCord::Cord(x,y) => (x + svg_origin.0,-y + svg_origin.1),
            PordOrCord::Pord(poi) => poi.svg_xy()
        }
    }
}


impl Polar for POrd {
    fn r(&self) -> Rc<f64> {self.r.clone()}
    fn theta(&self) -> Rc<f64> {self.theta.clone()}
    fn anchor(&self) -> Weak<PordOrCord> {
        self.anchor.clone()
    }
}

pub trait Cartesian {
    fn xy(&self) -> (f64,f64);
}
pub trait Polar:Cartesian {
    fn r(&self) -> Rc<f64>;
    fn theta(&self) -> Rc<f64>;
    fn anchor(&self) -> Weak<PordOrCord>;
    fn anchor_xy(&self) -> Option<(f64,f64)> {
        let poc = self.anchor().upgrade()?;
        Some(   match poc.as_ref() {
            PordOrCord::Cord(x,y) => {(*x,*y)}
            PordOrCord::Pord(p) => {p.xy()}
        })
    }
    fn svg_xy(&self) -> (f64,f64) {
        let (x,y) = self.anchor_xy().expect("Memory management is easy");
        let (x_rel, y_rel) = self.xy_rel();
        (x+x_rel,y-y_rel)
    } 
    fn xy_rel(&self) -> (f64, f64) {
        let y = -f64::cos(*self.theta())* *self.r();
        let x = f64::sin(*self.theta())* *self.r();
        (x, y)
    }
}

impl Cartesian for POrd {
    fn xy(&self) -> (f64,f64) {
        let (x,y) = self.anchor_xy().expect("Memory management is easy");
        let (x_rel, y_rel) = self.xy_rel();
        (x+x_rel,y+y_rel)
    }
}