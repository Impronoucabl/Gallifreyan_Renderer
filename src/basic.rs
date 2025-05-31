use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx as context;
use crate::pord::{PordOrCord,Polar};

fn p_or_c2svg(poc:&PordOrCord, svg_origin:(f64,f64)) -> (f64,f64) {
    match poc {
        PordOrCord::Cord(x,y) => (x + svg_origin.0,-y + svg_origin.1),
        PordOrCord::Pord(poi) => poi.svg_xy()
    }
}

pub fn circle(doc:Document, center:&PordOrCord, radius:f64, ctx:&context::Context) -> Document {
    let center = p_or_c2svg(center, ctx.origin());
    let circle = Circle::new()
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth())
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", radius);
    doc.add(circle)
}

pub fn arc_circle(doc:Document,start:&PordOrCord, end:&PordOrCord,radius:f64, sweep_dir:f64, ctx:&context::Context) -> Document {
    let start = p_or_c2svg(start, ctx.origin());
    let end = p_or_c2svg(end, ctx.origin());
    let data = Data::new()
        .move_to(start)
        .elliptical_arc_to((
            radius,radius,
            0.0, //angle offset
            0.0, //large arc
            sweep_dir,
            end.0,end.1,
        ));
    let arc = Path::new()
        .set("d", data)
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth());
    doc.add(arc)
}