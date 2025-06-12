use crate::types::gkg_table::SocialMediaEmbedUrl;

pub enum SocialEmbed {
    Image(SocialMediaEmbedUrl),
    Video(SocialMediaEmbedUrl),
}
