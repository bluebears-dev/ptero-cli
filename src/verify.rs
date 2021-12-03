pub trait Verify<Cover, Err> {
    fn can_hide_data(&self, cover: Cover) -> Result<(), Err>;
}