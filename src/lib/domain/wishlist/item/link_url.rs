use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemLinkUrl(url::Url);

impl From<&str> for ItemLinkUrl {
    fn from(value: &str) -> Self {
        let url = Url::parse(value).expect("Parse error");
        ItemLinkUrl(url)
    }
}
