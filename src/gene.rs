#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum Crossed {
    Crossed,
    Straight,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Coefficient {
    Excitatory,
    Inhibitory,
}

#[derive(Debug)]
pub struct Gene {
    pub sensor_side: Side,
    pub coefficient: Coefficient,
    pub engine_connection: Side,
    // stimulus: {light, vehicle, food, etc}
}

pub type Genes = Vec<Gene>;

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
            engine_connection: sensor_side,
        }
    }
    pub fn new_crossed(sensor_side: Side, coefficient: Coefficient) -> Self {
        Self {
            sensor_side,
            coefficient,
            engine_connection: sensor_side.cross(),
        }
    }
    pub fn sensor_and_engine_is_crossed(&self) -> bool {
        self.sensor_side == self.engine_connection
    }
}
