use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemImageUrl(url::Url);

impl From<&str> for ItemImageUrl {
    fn from(value: &str) -> Self {
        let url = Url::parse(value).expect("Parse error");
        ItemImageUrl(url)
    }
}
