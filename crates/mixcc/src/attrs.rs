#[non_exhaustive]
#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,Debug, Hash)]
pub enum MixCCAttribute{
    Musttail,
    NeverRecurses,
}