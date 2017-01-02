#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
use schema::Type;
use schema::Field;
use schema::Schema;

describe! schema_test {

    before_each {
        let types = vec![Type::new("Decimal128", 5, 0.75, 26)];
        let field = Field::new("testing", 5, 0.5, true, &types);
        let fields = vec![field];
        let schema = Schema::new(10, &fields);
    }

    it "exposes a count property" {
        assert_eq!(10, schema.count);
    }
}
