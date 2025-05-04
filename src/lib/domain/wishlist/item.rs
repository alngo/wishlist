mod image_url;
mod link_url;
mod price;
mod repository;
mod service;
mod title;

pub use image_url::ItemImageUrl;
pub use link_url::ItemLinkUrl;
pub use price::ItemPrice;
pub use repository::*;
pub use service::*;
pub use title::ItemTitle;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    id: Uuid,
    title: ItemTitle,
    link_url: ItemLinkUrl,
    image_url: ItemImageUrl,
    price: ItemPrice,
}

impl Item {
    pub fn create(
        id: Uuid,
        title: ItemTitle,
        link_url: ItemLinkUrl,
        image_url: ItemImageUrl,
        price: ItemPrice,
    ) -> Self {
        Self {
            id,
            title,
            link_url,
            image_url,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::domain::wishlist::{
        item::{ItemImageUrl, ItemLinkUrl, ItemPrice, ItemTitle},
        Item,
    };

    #[test]
    fn create_item() {
        let id = Uuid::now_v7();
        let title = ItemTitle::from("Title");
        let link_url = ItemLinkUrl::from("https://www.test_link_url.com");
        let image_url = ItemImageUrl::from("https://www.test_image_url.com");
        let price = ItemPrice::from(10.10);
        let item = Item::create(id, title, link_url, image_url, price);

        assert_eq!(item.id, id);
        assert_eq!(item.title, "Title".into());
        assert_eq!(item.link_url, "https://www.test_link_url.com".into());
        assert_eq!(item.image_url, "https://www.test_image_url.com".into());
        assert_eq!(item.price, 10.10.into());
    }
}
