// TODO add more options
// TODO: implement this configs
pub(super) struct YouTubeConfig {
    search_count: u8,
    age_restricted: bool
}

impl YouTubeConfig {
    pub fn generate(search_count: u8, age_restricted: bool) -> Self {
        Self { search_count, age_restricted }
    }

    pub fn search_count(&self) -> u8 {
        self.search_count
    }

    pub fn age_restricted(&self) -> bool {
        self.age_restricted
    }
}
