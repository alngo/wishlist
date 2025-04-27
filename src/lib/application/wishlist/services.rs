use super::repository::WishlistRepository;

pub struct WishlistService<R>
where
    R: WishlistRepository,
{
    repository: R,
}
