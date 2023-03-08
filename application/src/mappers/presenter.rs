pub trait Presenter<Entity> {
    fn to_api(entity: &Entity) -> Self;
}
