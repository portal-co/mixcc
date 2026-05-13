use alloc::{collections::btree_set::BTreeSet, string::String, vec::Vec};

use crate::attrs::MixCCAttribute;
macro_rules! id {
    ($a:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub struct $a(pub usize);
    };
}
id!(FunctionId);
id!(BlockId);
id!(ValueId);
pub struct Function {
    pub name: String,
    pub attrs: BTreeSet<MixCCAttribute>,
    pub body: Option<Body>,
}
pub struct CompilationUnit {
    pub functions: Vec<Function>,
}
pub struct Body {
    pub values: Vec<(Value, Option<usize>)>,
    pub blocks: Vec<Block>,
    pub entry: BlockId,
}
pub enum Value {
    Param { block: BlockId, index: usize },
}
pub struct Block {
    pub params: Vec<ValueId>,
    pub values: Vec<ValueId>,
    pub terminator: Terminator,
}
pub enum Terminator {
    Jmp(Target),
    Tail {
        func: CallTarget,
        args: Vec<ValueId>,
    },
}
pub struct Target {
    pub block: BlockId,
    pub args: Vec<ValueId>,
}
id!(CallTarget);
impl CallTarget {
    pub fn direct(func: FunctionId) -> Self {
        Self(func.0 << 1)
    }
    pub fn indirect(func: ValueId) -> Self {
        Self((func.0 << 1) | 1)
    }
    pub fn is_direct(&self) -> bool {
        self.0 & 1 == 0
    }
    pub fn is_indirect(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn function(&self) -> FunctionId {
        FunctionId(self.0 >> 1)
    }
    pub fn value(&self) -> ValueId {
        ValueId(self.0 >> 1)
    }
}
