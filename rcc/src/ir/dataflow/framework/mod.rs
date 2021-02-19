mod lattice;

pub enum Direction {
    Forward,
    Backward
}

pub trait DataflowAnalysis {
    type Lattice;
    type TransferFunctions;
    fn direction(&self) -> Direction;

}