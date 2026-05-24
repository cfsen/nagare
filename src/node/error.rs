pub enum NodeError {
    GetInput(String),
    GetParam(String),
    SetIONodeNotFound,
    SetIOKindMismatch,
}
