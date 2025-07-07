#[derive(Clone, Copy, Debug)]
pub enum Throughput {
    Bytes(usize),
    Items(usize),
}
