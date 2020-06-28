use crate::components::types::statics::trafficlight::TrafficLightColor;

#[derive(Clone, Debug)]
pub enum Event {
    TrafficLightColorChange(TrafficLightColor),
}
