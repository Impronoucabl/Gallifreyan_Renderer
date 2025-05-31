use svg::node::element::path::{Command, Data};
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx as context;

pub fn circle(doc:Document, center:(f64,f64), radius:f64, ctx:&context::Context) -> Document {
    let center = (center.0 + ctx.origin().0,-center.1 + ctx.origin().1);
    let circle = Circle::new()
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth())
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", radius);
    doc.add(circle)
}

pub fn arc_circle(doc:Document,start:(f64,f64), end:(f64,f64),radius:f64, ctx:&context::Context) -> Document {
    let start = (start.0 + ctx.origin().0,ctx.origin().1 - start.1);
    let end = (end.0 + ctx.origin().0,ctx.origin().1 - end.1);
    let data = Data::new()
        .move_to(start)
        .elliptical_arc_to((
            radius,radius,
            0.0, //angle offset
            0.0, //large arc
            1.0, //sweep dir
            end.0,end.1,
        ));
    let arc = Path::new()
        .set("d", data)
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth());
    doc.add(arc)
}