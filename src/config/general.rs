pub(super) struct GeneralConfig {
    token: String,
    prefix: String,
    vc_auto_change: bool,
}

impl GeneralConfig {
    pub fn generate(token: String, prefix: String, vc_auto_change: bool) -> Self {
        Self { token, prefix, vc_auto_change }
    }

    #[inline(always)]
    pub fn token(&self) -> &String {
        &self.token
    }

    #[inline(always)]
    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    #[inline(always)]
    pub fn vc_auto_change(&self) -> bool {
        self.vc_auto_change
    }
}

