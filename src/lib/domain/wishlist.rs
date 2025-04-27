mod item;
mod name;
mod slug;

pub use item::Item;
pub use name::WishlistName;
pub use slug::WishlistSlug;
use std::collections::HashSet;
use uuid::Uuid;

pub struct Wishlist {
    id: Uuid,
    owner_id: Uuid,
    name: WishlistName,
    slug: WishlistSlug,
    private: bool,
    items: HashSet<Item>,
}

impl Wishlist {
    pub fn create(
        id: Uuid,
        owner_id: Uuid,
        name: WishlistName,
        slug: WishlistSlug,
        private: bool,
    ) -> Self {
        Self {
            id,
            owner_id,
            name,
            slug,
            private,
            items: HashSet::new(),
        }
    }

    pub fn add_item(&mut self, _item: Item) {
        todo!()
    }

    pub fn remove_item(&mut self, _item_id: Uuid) {
        todo!()
    }
}

#[cfg(test)]
mod user_tests {
    use super::{Wishlist, WishlistName, WishlistSlug};
    use uuid::Uuid;

    #[test]
    fn create_wishlist() {
        let id = Uuid::now_v7();
        let owner_id = Uuid::now_v7();
        let name = WishlistName::from("Test wishlist");
        let slug = WishlistSlug::from("Wishlist");
        let wishlist = Wishlist::create(id, owner_id, name, slug, true);

        assert_eq!(wishlist.id, id);
        assert_eq!(wishlist.owner_id, owner_id);
        assert_eq!(wishlist.name, "Test wishlist".into());
        assert_eq!(wishlist.private, true);
        assert!(wishlist.items.is_empty());
    }
}
