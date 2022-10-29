use std::fmt::Display;

pub(super) struct Song {
    title: String,
    url: String,
    length: String,
    user_name: String,
}

impl Song {
    pub async fn new<A, B>(url: A, user_name: B) -> Self
    where
        A: Into<String>,
        B: Into<String>
    {
        let s = {
            let u = url.into();

            if u.contains("youtube.com") {
                Self::yt_url(u).await
            } else {
                Self::sp_url(u).await
            }
        };

        Self {
            title: s.0,
            url: s.1,
            length: s.2,
            user_name: user_name.into(),
        }
    }

    #[inline(always)]
    async fn yt_url(url: String) -> (String, String, String) {
        todo!("Handle YouTube URL")
    }

    #[inline(always)]
    async fn sp_url(url: String) -> (String, String, String) {
        todo!("Handle Spotify URL")
    }

    #[inline(always)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[inline(always)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[inline(always)]
    pub fn length(&self) -> String {
        self.length.clone()
    }

    #[inline(always)]
    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.title, self.length)
    }
}

