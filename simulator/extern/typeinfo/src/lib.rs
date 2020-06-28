pub trait TypeInfo {
    const TYPENAME: &'static str;
    fn type_of(&self) -> &'static str {
        Self::TYPENAME
    }
    fn typestring() -> String {
        String::from(Self::TYPENAME)
    }
}
