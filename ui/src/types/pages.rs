use enum_iterator::Sequence;

#[derive(Debug, Clone, Default, PartialEq, Eq, Sequence, strum::Display)]
pub enum Page {
    #[default]
    Proxy,
    Brute,
    Replay,
    Scan,
    Discover,
}
