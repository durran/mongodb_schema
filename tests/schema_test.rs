#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
use schema::Type;
use schema::Field;
use schema::Schema;

describe! schema_test {

    describe! new {

        before_each {
            let mut types = Vec::new();
            let mut fields = Vec::new();
            types.push(Type::new("Decimal128".to_string(), 5, 0.75, 26));
            fields.push(Field::new("testing".to_string(), 5, 0.5, true, types));
            let schema = Schema::new(10, fields);
        }

        it "exposes a count property" {
            assert_eq!(10, schema.count);
        }
    }
}
