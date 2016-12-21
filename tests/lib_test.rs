#[cfg(test)]

extern crate mongodb_schema;

/// Tests for the mongodb schema module.
mod test {
    use mongodb_schema::Type as Type;

    #[test]
    fn type_must_expose_name() {
        let t = Type::new("Decimal128", 5, 0.75, 26);
        assert_eq!("Decimal128", t.name);
    }

    #[test]
    fn type_must_expose_count() {
        let t = Type::new("Decimal128", 5, 0.75, 26);
        assert_eq!(5, t.count);
    }

    #[test]
    fn type_must_expose_probability() {
        let t = Type::new("Decimal128", 5, 0.75, 26);
        assert_eq!(0.75, t.probability);
    }

    #[test]
    fn type_must_expose_unique() {
        let t = Type::new("Decimal128", 5, 0.75, 26);
        assert_eq!(26, t.unique);
    }
}
