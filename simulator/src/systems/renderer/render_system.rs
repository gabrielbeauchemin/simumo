extern crate graphics;
use crate::components::constant::Drawer;
use crate::components::statics::trafficlight::Light;
use crate::components::types::constant::CarType;
use crate::components::Position;
use crate::configurations::debugger::VisualDebugger;
use crate::ressources::generals::MapBbox;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::color::Color;
use crate::systems::renderer::drawableshape::Drawable;
use graphics::{clear, rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect};

const EDGE_WIDTH: f64 = 1.0;

pub struct DrawClear;

impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::GREENFOREST.get(), gl);
        });
    }
}

pub struct DrawMap;

impl<'a> System<'a> for DrawMap {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadExpect<'a, LaneGraph>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (debugger, map_bbox, lane_graph, mut g_handle, args): Self::SystemData) {
        for edge in lane_graph.lanes().all_edges() {
            let node1 = lane_graph.intersection(edge.0);
            let node2 = lane_graph.intersection(edge.1);
            let pos_node1: (f64, f64) = point_to_window(node1.position(), &debugger, &map_bbox);
            let pos_node2: (f64, f64) = point_to_window(node2.position(), &debugger, &map_bbox);
            g_handle.draw(args.viewport(), |c, gl| {
                draw_lane_between_two_points(
                    pos_node1,
                    pos_node2,
                    EDGE_WIDTH,
                    Color::LIGHTGRAY,
                    c,
                    gl,
                );
            });
        }
    }
}

pub struct DrawTrafficLights;

impl<'a> System<'a> for DrawTrafficLights {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
        ReadExpect<'a, LaneGraph>,
    );

    fn run(
        &mut self,
        (debugger, map_bbox, positions, lights, drawers, mut g_handle, args, lane_graph): Self::SystemData,
    ) {
        for (position, light, drawer) in (&positions, &lights, &drawers).join() {
            if let Some((x, y)) = pos_to_window(position, &debugger, &map_bbox, &lane_graph) {
                g_handle.draw(args.viewport(), |c, gl| {
                    drawer
                        .figure
                        .draw(x, y, light.color.get_rendering_color(), c, gl);
                });
            }
        }
    }
}

pub struct DrawVehicles;

impl<'a> System<'a> for DrawVehicles {
    type SystemData = (
        ReadExpect<'a, VisualDebugger>,
        ReadExpect<'a, MapBbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, CarType>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
        ReadExpect<'a, LaneGraph>,
    );

    fn run(
        &mut self,
        (debugger, map_bbox, positions, cars, drawers, mut g_handle, args, lane_graph): Self::SystemData,
    ) {
        for (position, _car, drawer) in (&positions, &cars, &drawers).join() {
            if let Some((x, y)) = pos_to_window(position, &debugger, &map_bbox, &lane_graph) {
                debug!("vehicule rendering: x={} y={}", x, y);
                g_handle.draw(args.viewport(), |c, gl| {
                    drawer.figure.draw(x, y, Color::BLACK, c, gl);
                });
            }
        }
    }
}

fn draw_lane_between_two_points(
    p1: (f64, f64),
    p2: (f64, f64),
    width: f64,
    color: Color,
    c: Context,
    gl: &mut GlGraphics,
) {
    let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
    let rectangle_width: f64 = width;
    let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);

    let transform = c
        .transform
        .trans(p1.0, p1.1)
        .rot_rad(rectangle_angle)
        .scale(rectangle_length, rectangle_width);
    rectangle(color.get(), rectangle::square(0.0, 0.0, 1.0), transform, gl);
}

fn pos_to_window(
    pos: &Position,
    debugger: &VisualDebugger,
    map_bbox: &MapBbox,
    lane_graph: &LaneGraph,
) -> Option<(f64, f64)> {
    if let Some(lane) = lane_graph.lane_between(pos.val.0) {
        let cpoint = lane.curve().get_location_at_percentage(pos.val.1);
        return Some(point_to_window(
            (cpoint.point().x, cpoint.point().y),
            debugger,
            map_bbox,
        ));
    }
    None
}

pub fn point_to_window(
    (x, y): (f64, f64),
    debugger: &VisualDebugger,
    map_bbox: &MapBbox,
) -> (f64, f64) {
    let diff_x: f64 = map_bbox.x2 - map_bbox.x1;
    let diff_y: f64 = map_bbox.y2 - map_bbox.y1;
    let width: f64 = debugger.width;
    let height: f64 = debugger.height;
    let xpx = width * (x - map_bbox.x1) / diff_x;
    let ypx = height * (map_bbox.y2 - y) / diff_y;
    (xpx, ypx)
}
