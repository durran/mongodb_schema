extern crate schema;

/// Tests for the mongodb schema module.
mod test {
    use schema::Type as Type;
    use schema::Field as Field;
    use schema::Schema as Schema;

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

    #[test]
    fn field_must_expose_name() {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        assert_eq!("testing", field.name);
    }

    #[test]
    fn field_must_expose_count() {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        assert_eq!(5, field.count);
    }

    #[test]
    fn field_must_expose_probability() {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        assert_eq!(0.5, field.probability);
    }

    #[test]
    fn field_must_expose_has_duplicates() {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        assert_eq!(true, field.has_duplicates);
    }

    #[test]
    fn schema_must_expose_count() {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        let fields = vec![field];
        let schema = Schema::new(10, &fields);
        assert_eq!(10, schema.count);
    }
}
