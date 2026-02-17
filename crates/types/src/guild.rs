#[derive(Clone, Debug)]
pub struct GuildId(pub u64);

#[derive(Clone, Debug)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub description: String,
}
