#[derive(Debug,Clone)]
pub enum Message {
    CellLeftClick(usize, usize),
    CellRightClick(usize, usize),
    Restart,
    
}