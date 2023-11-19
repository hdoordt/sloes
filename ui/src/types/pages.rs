use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Eq, Sequence, strum::Display)]
pub enum Page {
    Proxy,
    Brute,
    Replay,
    Scan,
    Discover,
}
