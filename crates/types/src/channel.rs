#[derive(Clone, Debug)]
pub struct ChannelId(pub u64);

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
}
