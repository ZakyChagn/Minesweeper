#[derive(Debug,Clone)]
pub enum Message {
    CellPressed(usize, usize),
    Restart,
}