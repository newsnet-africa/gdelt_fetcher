use super::ToRequestLink;
use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;

impl ToRequestLink for GDELTCategoryList {
    fn to_request_link(&self) -> String {
        self.to_string()
    }
}
