use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Cartesian, PordOrCord};
use crate::utils::SweepDirection;

pub fn circle(doc:Document, center:&PordOrCord, radius:f32, ctx:&Context) -> Document {
    let center = center.abs_svg_xy(ctx.origin());
    let circle = Circle::new()
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth())
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", radius);
    doc.add(circle)
}

pub fn arc_big_circle(doc:Document,start:&PordOrCord, end:&PordOrCord,radius:f32, sweep_dir:SweepDirection, ctx:&Context) -> Document {
    let start = start.abs_svg_xy(ctx.origin());
    let end = end.abs_svg_xy(ctx.origin());
    let data = Data::new()
        .move_to(start)
        .elliptical_arc_to((
            radius,radius,
            0.0, //angle offset
            1.0, //large arc
            match sweep_dir.0 {
                false => 0.0,
                true => 1.0
            },
            end.0,end.1,
        ));
    let arc = Path::new()
        .set("d", data)
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth());
    doc.add(arc)
}

pub fn arc_small_circle(doc:Document,start:&PordOrCord, end:&PordOrCord,radius:f32, sweep_dir:SweepDirection, ctx:&Context) -> Document {
    let start = start.abs_svg_xy(ctx.origin());
    let end = end.abs_svg_xy(ctx.origin());
    let data = Data::new()
        .move_to(start)
        .elliptical_arc_to((
            radius,radius,
            0.0, //angle offset
            0.0, //large arc
            match sweep_dir.0 {
                false => 0.0,
                true => 1.0
            },
            end.0,end.1,
        ));
    let arc = Path::new()
        .set("d", data)
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth());
    doc.add(arc)
}

pub fn arc_path(doc:Document,thickness:f32, start:&PordOrCord, end:&PordOrCord,radius:f32, sweep_dir:SweepDirection, ctx:&Context) -> Document {
    let start = start.abs_svg_xy(ctx.origin());
    let end = end.abs_svg_xy(ctx.origin());
    let (o_radius, i_radius) = (radius+thickness,radius-thickness);
    let data = Data::new()
        .move_to(start)
        .elliptical_arc_to((
            o_radius,o_radius,
            0.0, //angle offset
            0.0, //large arc
            match sweep_dir.0 {
                false => 0.0,
                true => 1.0
            },
            end.0,end.1,
        )).elliptical_arc_to((
            i_radius,i_radius,
            0.0, //angle offset
            0.0, //large arc
            match sweep_dir.0 { //swap direction
                false => 1.0,
                true => 0.0
            },
            start.0,start.1,
        )).close();
    let arc = Path::new()
        .set("d", data)
        .set("fill", ctx.colour().fill())
        .set("stroke", ctx.colour().stroke())
        .set("stroke-width", ctx.stroke().strokewidth());
    doc.add(arc)
}