use math::coordinate::Position;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait Entity {
    fn get_position(&self) -> Position;
    fn get_entity_type(&self) -> registries::entity_type::EntityTypeRegistry;
}
