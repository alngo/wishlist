mod item;
mod name;
mod repository;
mod service;
mod slug;

pub use item::*;
pub use name::*;
pub use repository::*;
pub use service::*;
pub use slug::WishlistSlug;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Wishlist {
    id: Uuid,
    owner_id: Uuid,
    name: WishlistName,
    slug: WishlistSlug,
    private: bool,
    items: HashSet<Uuid>,
}

impl Wishlist {
    pub fn new(
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    pub fn name(&self) -> &WishlistName {
        &self.name
    }

    pub fn slug(&self) -> &WishlistSlug {
        &self.slug
    }

    pub fn private(&self) -> bool {
        self.private
    }

    pub fn items(&self) -> &HashSet<Uuid> {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::{Wishlist, WishlistName, WishlistSlug};
    use uuid::Uuid;

    #[test]
    fn create_wishlist() {
        let id = Uuid::now_v7();
        let owner_id = Uuid::now_v7();
        let name = WishlistName::from("Test wishlist");
        let slug = WishlistSlug::from("Test Wishlist");
        let wishlist = Wishlist::new(id, owner_id, name, slug, true);

        assert_eq!(wishlist.id, id);
        assert_eq!(wishlist.owner_id, owner_id);
        assert_eq!(wishlist.name, "Test wishlist".into());
        assert!(wishlist.slug.to_string().contains("test-wishlist-"));
        assert_eq!(wishlist.private, true);
        assert!(wishlist.items.is_empty());
    }
}
