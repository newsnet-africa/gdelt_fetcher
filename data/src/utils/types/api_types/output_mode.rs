use super::ToRequestLink;

pub enum OutputMode {
    ArtList,
    ArtGallery,
    ImageCollage,
    ImageCollageInfo,
    ImageGallery,
    ImageCollageShare,
    TimelineVol,
    TimelineVolRow,
    TimelineVolInfo,
    TimelineTone,
    TimelineLang,
    TimelineSourceCountry,
    ToneChart,
    WordCloudImageTags,
    WordCloudImageWebTags,
}

impl ToRequestLink for OutputMode {
    fn to_request_link(&self) -> String {
        match self {
            Self::ArtList => "artlist".to_string(),
            Self::ArtGallery => "artgallery".to_string(),
            Self::ImageCollage => "imagecollage".to_string(),
            Self::ImageCollageInfo => "imagecollageinfo".to_string(),
            Self::ImageGallery => "imagegallery".to_string(),
            Self::ImageCollageShare => "imagecollageshare".to_string(),
            Self::TimelineVol => "timelinevol".to_string(),
            Self::TimelineVolRow => "timelinevolrow".to_string(),
            Self::TimelineVolInfo => "timelinevolinfo".to_string(),
            Self::TimelineTone => "timelinetone".to_string(),
            Self::TimelineLang => "timelinelang".to_string(),
            Self::TimelineSourceCountry => "timelinesourcecountry".to_string(),
            Self::ToneChart => "tonechart".to_string(),
            Self::WordCloudImageTags => "wordcloudimagetags".to_string(),
            Self::WordCloudImageWebTags => "wordcloudimagewebtags".to_string(),
        }
    }
}
