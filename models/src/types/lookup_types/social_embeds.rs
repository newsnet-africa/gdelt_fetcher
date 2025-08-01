use crate::types::gkg_table::SocialMediaEmbedUrl;

#[derive(Debug)]
pub enum SocialEmbed {
    Image(SocialMediaEmbedUrl),
    Video(SocialMediaEmbedUrl),
}
