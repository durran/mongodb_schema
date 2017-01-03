#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
use schema::Type;

describe! type_test {

    describe! new {

        before_each {
            let t = Type::new("Decimal128".to_string(), 5, 0.75, 26);
        }

        it "exposes a name property" {
            assert_eq!("Decimal128", t.name);
        }

        it "exposes a count property" {
            assert_eq!(5, t.count);
        }

        it "exposes a probability property" {
            assert_eq!(0.75, t.probability);
        }

        it "exposes a unique property" {
            assert_eq!(26, t.unique);
        }
    }
}
