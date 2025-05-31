#[derive(Clone)]
pub struct Context {
    colour: ColourContext,
    stroke: StrokeContext,
}
#[derive(Clone)]
pub struct ColourContext {
    fill:String,
    stroke:String,
}
#[derive(Clone, Copy)]
pub struct StrokeContext {
    strokewidth:f64
}

impl ColourContext {
    pub fn new(fill:&str,stroke:&str) -> ColourContext {
        ColourContext { 
            fill:fill.to_string(), 
            stroke:stroke.to_string(), 
        }
    }
    pub fn fill(&self) -> &str {
        &self.fill
    }
    pub fn stroke(&self) -> &str {
        &self.stroke
    }
}

impl StrokeContext {
    pub fn new(strokewidth:f64) -> StrokeContext {
        StrokeContext { strokewidth }
    }
    pub fn strokewidth(&self) -> f64 {
        self.strokewidth
    }
}

impl Context {
    pub fn new(colour:ColourContext, stroke:StrokeContext) -> Context {
        Context { colour, stroke }
    }
    pub fn colour(&self) -> &ColourContext {
        &self.colour
    }
    pub fn stroke(&self) -> &StrokeContext {
        &self.stroke
    }
}