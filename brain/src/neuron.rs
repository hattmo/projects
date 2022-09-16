pub struct Neuron {
    charge: u8,
    ceiling: u8,
    floor: u8,
    pressure: u8,
    output: u8,
    conns: [Conn; 16],
}

pub struct Conn {
    weight: u8,
    neuron: usize,
}

pub struct Network<const T: usize> {
    neurons: [Neuron; T],
}
