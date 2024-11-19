use super::ToRequestLink;

pub enum Translator {
    Google,
}

impl ToRequestLink for Translator {
    fn to_request_link(&self) -> String {
        match self {
            Translator::Google => "googtrans".to_string(),
        }
    }
}
