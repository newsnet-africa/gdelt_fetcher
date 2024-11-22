use super::ToRequestLink;
use models::models::gdelt::utils::gdelt_languages::GDELTLanguage;

impl ToRequestLink for GDELTLanguage {
    fn to_request_link(&self) -> String {
        self.code().to_string()
    }
}
