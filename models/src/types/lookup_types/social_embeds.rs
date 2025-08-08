use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum SocialEmbed {
    Image(Url),
    Video(Url),
}

impl SocialEmbed {
    /// Create a new image embed
    pub fn image(url: Url) -> Self {
        Self::Image(url)
    }

    /// Create a new video embed
    pub fn video(url: Url) -> Self {
        Self::Video(url)
    }

    /// Get the URL regardless of embed type
    pub fn url(&self) -> &Url {
        match self {
            Self::Image(url) | Self::Video(url) => url,
        }
    }

    /// Check if this is an image embed
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    /// Check if this is a video embed
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video(_))
    }
}

impl From<Url> for SocialEmbed {
    fn from(url: Url) -> Self {
        // Default to video embed - this could be refined based on URL analysis
        Self::Video(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_embed_creation() {
        let url = Url::parse("https://youtube.com/watch?v=abc123").unwrap();
        let embed = SocialEmbed::video(url.clone());

        assert!(embed.is_video());
        assert!(!embed.is_image());
        assert_eq!(embed.url(), &url);
    }

    #[test]
    fn test_social_embed_from_url() {
        let url = Url::parse("https://instagram.com/p/abc123").unwrap();
        let embed = SocialEmbed::from(url.clone());

        assert_eq!(embed.url(), &url);
    }
}
