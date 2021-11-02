enum light {
    Red,
    Green,
    Yellow,
}
//setup trait
pub trait GetTrafficLightTime {
    fn set_traffic_time(&self)->u32;
}
//implement trait for different traffic light
impl GetTrafficLightTime for light{
    fn set_traffic_time(&self)->u32{
        match &self {
            light::Green => 35,
            light::Red => 20,
            light::Yellow => 5,
        }
    }
}

fn main() {
    let red = light::Red;
    println!("Red light will last for: {} second", red.set_traffic_time());
    let green = light::Green;
    println!("Green light will last for: {} second", green.set_traffic_time());
    let yellow = light::Yellow;
    println!("Yellow light will last for: {} second", yellow.set_traffic_time());
}