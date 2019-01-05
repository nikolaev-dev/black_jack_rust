pub struct Card {
    pub value: i32,
    pub name: String
}



impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            value: self.value,
            name: self.name.clone()
        }
    }
}