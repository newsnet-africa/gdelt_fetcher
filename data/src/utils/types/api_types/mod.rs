use models::models::gdelt::utils::gdelt_languages::GDELTLanguage;

pub mod category_list;
pub mod json_types;
pub mod language;
pub mod operation;
pub mod output_format;
pub mod output_mode;
pub mod query_types;
pub mod rss_types;
pub mod sort_types;
pub mod translator;

pub trait ToRequestLink {
    fn to_request_link(&self) -> String;
}

// impl ToRequestLink for GDELTLanguage {
//     fn to_request_link(&self) -> String {
//         format!(
//             "https://api.gdeltproject.org/api/v2/doc/doc?query&lang={}",
//             self.code()
//         )
//     }
// }
