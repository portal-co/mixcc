use crate::id;

id!(Symbol);
pub enum WorkspaceKind{
    Stack,
    Static{
        symbol: Symbol,
    }
}