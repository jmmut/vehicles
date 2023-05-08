#[derive(PartialEq, Copy, Clone)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn cross(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Crossed {
    Crossed,
    Straight,
}

#[derive(PartialEq, Copy, Clone)]
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

impl Gene {
    pub fn new(crossed: Crossed, sensor_side: Side, coefficient: Coefficient) -> Self {
        match crossed {
            Crossed::Crossed => Self::new_crossed(sensor_side, coefficient),
            Crossed::Straight => Self::new_straight(sensor_side, coefficient),
        }
    }
    pub fn new_straight(sensor_side: Side, coefficient: Coefficient) -> Self {
        Self {
            sensor_side,
            coefficient,
            motor_connection: sensor_side,
        }
    }
    pub fn new_crossed(sensor_side: Side, coefficient: Coefficient) -> Self {
        Self {
            sensor_side,
            coefficient,
            motor_connection: sensor_side.cross(),
        }
    }
    pub fn sensor_and_motor_is_crossed(&self) -> bool {
        self.sensor_side == self.motor_connection
    }
}
