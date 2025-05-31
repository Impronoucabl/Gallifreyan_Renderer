use svg::Document;
use svg::node::element::{Circle};

use crate::ctx as context;

pub fn circle(doc:Document, center:(f64,f64), radius:f64, ctx:&context::Context) -> Document {
    let circle = Circle::new()
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth())
        .set("cx", center.0 + ctx.origin().0)
        .set("cy", -center.1 + ctx.origin().1)
        .set("r", radius);
    doc.add(circle)
}