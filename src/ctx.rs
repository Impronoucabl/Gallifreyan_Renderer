#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    colour: ColourContext,
    stroke: StrokeContext,
    origin: (f64,f64)
}
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ColourContext {
    bg:String,
    fill:String,
    stroke:String,
}
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct StrokeContext {
    inner_strokewidth:f64,
    outer_strokewidth:f64,
}

impl Default for ColourContext {
    fn default() -> Self {
        Self { 
            bg: "white".to_string(), 
            fill: "none".to_string(), 
            stroke: "black".to_string()
        }
    }
}

impl ColourContext {
    pub fn new(bg:&str,fill:&str,stroke:&str) -> ColourContext {
        ColourContext { 
            bg:bg.to_string(),
            fill:fill.to_string(), 
            stroke:stroke.to_string(), 
        }
    }
    pub fn default_path() -> Self {
        ColourContext { 
            bg: "white".to_string(), 
            fill: "black".to_string(), 
            stroke: "none".to_string() 
        }
    }
    pub fn bg(&self) -> &str {
        &self.bg
    }
    pub fn fill(&self) -> &str {
        &self.fill
    }
    pub fn stroke(&self) -> &str {
        &self.stroke
    }
}

impl StrokeContext {
    //Might need to revise where I divide by 2
    pub fn new(strokewidth:f64) -> StrokeContext {
        StrokeContext { 
            inner_strokewidth: strokewidth/2.0,
            outer_strokewidth: strokewidth/2.0,
        }
    }
    pub fn strokewidth(&self) -> f64 {
        self.inner_strokewidth + self.outer_strokewidth
    }
    pub fn i_stroke(&self) -> f64 {
        self.inner_strokewidth
    }
    pub fn o_stroke(&self) -> f64 {
        self.outer_strokewidth
    }
    pub fn set_i_stroke(&mut self, new_i_stroke:f64) {
        self.inner_strokewidth = new_i_stroke;
    }
    pub fn set_o_stroke(&mut self, new_o_stroke:f64) {
        self.outer_strokewidth = new_o_stroke;
    }
}

impl Context {
    pub fn new(colour:ColourContext, stroke:StrokeContext, origin:&(f64,f64)) -> Context {
        Context { colour, stroke, origin:*origin }
    }
    pub fn colour(&self) -> &ColourContext {
        &self.colour
    }
    pub fn stroke(&self) -> &StrokeContext {
        &self.stroke
    }
    pub fn origin(&self) -> (f64,f64) {
        self.origin
    }
    pub fn new_strokewidth(&self, strokewidth:f64) -> Context {
        Context { colour: self.colour.clone(), stroke: StrokeContext::new(strokewidth) , origin:self.origin() }
    }
    pub fn set_origin(&mut self, svg_origin: (f64, f64)) {
        self.origin = svg_origin;
    }
}