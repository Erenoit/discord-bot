pub(super) struct GeneralConfig {
    token: String,
    prefix: String,
    vc_auto_change: bool,
}

impl GeneralConfig {
    pub fn generate(token: String, prefix: String) -> Self {
        Self { token, prefix }
    }

    #[inline(always)]
    pub fn token(&self) -> &String {
        &self.token
    }

    #[inline(always)]
    pub fn prefix(&self) -> &String {
        &self.prefix
    }

}

