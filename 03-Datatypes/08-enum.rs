enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;

    match light {
        // have to handle all cases
        TrafficLight::Red => println!("Stop!!!"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go!!!"),
    }

    // Stop!!!
}
