#[derive(Debug, Clone)]
pub struct SimulationContext {
    pub dim1_size: u32,
    pub dim2_size: u32,
    pub dim3_size: u32,
    pub dim4_size: u32,
    pub state: SimulationState,
}

impl SimulationContext {
    pub fn new(d1: u32, d2: u32, d3: u32, d4: u32, fill: Dim) -> Self {
        let state = match fill {
            Dim::D1 => SimulationState {
                dim1_fill: d1,
                ..Default::default()
            },
            Dim::D2 => SimulationState {
                dim2_fill: d2,
                ..Default::default()
            },
            Dim::D3 => SimulationState {
                dim3_fill: d3,
                ..Default::default()
            },
            Dim::D4 => SimulationState {
                dim4_fill: d4,
                ..Default::default()
            },
        };

        Self {
            dim1_size: d1,
            dim2_size: d2,
            dim3_size: d3,
            dim4_size: d4,
            state,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Dim {
    D1,
    D2,
    D3,
    D4,
}

#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    pub dim1_fill: u32,
    pub dim2_fill: u32,
    pub dim3_fill: u32,
    pub dim4_fill: u32,
}
