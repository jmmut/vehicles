pub enum Side {
    Left,
    Right,
}

pub enum Coefficient {
    Excitatory,
    Inhibitory,
}

pub struct Gene {
    pub sensor_side: Side,
    pub coefficient: Coefficient,
    pub motor_connection: Side,
    // stimulus: {light, vehicle, food, etc}
}
