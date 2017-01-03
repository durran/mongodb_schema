extern crate serde_json;

use std::collections::BTreeMap;
use serde_json::Value;

/// Represents a value type in a field.
///
/// # Fields
///
/// * `name` - The name of the BSON type.
/// * `count` - The number of occurences for the field.
/// * `probability` - The probability in the current field.
/// * `unique` - The count of unique values.
pub struct Type {
    pub name: String,
    pub count: i64,
    pub probability: f32,
    pub unique: i64
}

/// The Type implementation.
impl Type {

    /// Instantiate a new type.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the BSON type.
    /// * `count` - The number of occurences for the field.
    /// * `probability` - The probability in the current field.
    /// * `unique` - The count of unique values.
    ///
    /// # Returns
    ///
    /// A new Type.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(schema::Type::new("Decimal128".to_string(), 5, 0.75, 26).name, "Decimal128");
    /// ```
    pub fn new(name: String, count: i64, probability: f32, unique: i64) -> Type {
        Type {
            name: name,
            count: count,
            probability: probability,
            unique: unique
        }
    }
}

/// Represents field analysis in a MongoDB schema.
///
/// # Fields
///
/// * `name` - The name of the field.
/// * `count` - The number of documents with this field.
/// * `probability` - The probability of the field existing in a document.
/// * `has_duplicates` - If duplicate values of the field exist across documents.
/// * `types` - The encountered types of this field.
pub struct Field<'f> {
    pub name: String,
    pub count: i64,
    pub probability: f32,
    pub has_duplicates: bool,
    pub types: &'f Vec<Type>
}

/// The field implementation.
impl<'f> Field<'f> {

    /// Instantiate a new field.
    ///
    /// # Parameters
    ///
    /// * `name` - The name of the field.
    /// * `count` - The number of documents with this field.
    /// * `probability` - The probability of the field existing in a document.
    /// * `has_duplicates` - If duplicate values of the field exist across documents.
    /// * `types` - The encountered types of this field.
    ///
    /// # Returns
    ///
    /// A new Field.
    pub fn new(
        name: String,
        count: i64,
        probability: f32,
        has_duplicates: bool,
        types: &'f Vec<Type>) -> Field<'f> {

        Field {
            name: name,
            count: count,
            probability: probability,
            has_duplicates: has_duplicates,
            types: types
        }
    }
}

/// Represents a MongoDB schema.
///
/// # Fields
///
/// * `count` - The number of documents analysed.
/// * `fields` - The various fields in the schema.
pub struct Schema<'s> {
    pub count: usize,
    pub fields: &'s Vec<Field<'s>>
}

/// The MongoDB Schema implementation.
impl<'s> Schema<'s> {

    /// Instantiate a new schema.
    ///
    /// # Parameters
    ///
    /// * `count` - The total number of documents analysed.
    /// * `fields` - The various fields in the schema.
    ///
    /// # Returns
    ///
    /// A new schema.
    pub fn new(count: usize, fields: &'s Vec<Field<'s>>) -> Schema {
        Schema {
            count: count,
            fields: fields
        }
    }
}

/// Analyses documents to generate a schema.
///
/// # Fields
///
/// - `documents` - The JSON documents.
pub struct Analyser {
    pub documents: Value
}

/// The analyser implementation.
impl Analyser {

    /// Create the new Analyser with the JSON string.
    ///
    /// # Parameters
    ///
    /// * `documents` - The JSON string of documents to analyse.
    ///
    /// # Returns
    ///
    /// A new analyser.
    pub fn new(documents: &str) -> Analyser {
        Analyser {
            documents: serde_json::from_str(documents).unwrap()
        }
    }

    /// Run the analysis on the JSON documents.
    ///
    /// # Returns
    ///
    /// An analysed schema.
    pub fn run(&self) {
        let mut fields = BTreeMap::new();
        let docs = self.documents.as_array().unwrap();
        for document in docs {
            let doc = document.as_object().unwrap();
            for (name, value) in doc.iter() {
                match *value {
                    Value::Null => self.generate_field(&fields, name, "Null"),
                    Value::Bool(_) => self.generate_field(&fields, name, "Boolean"),
                    Value::I64(_) => self.generate_field(&fields, name, "Int64"),
                    Value::U64(_) => self.generate_field(&fields, name, "UInt64"),
                    Value::F64(_) => self.generate_field(&fields, name, "Double"),
                    Value::String(_) => self.generate_field(&fields, name, "String"),
                    Value::Array(_) => self.generate_field(&fields, name, "Arrray"),
                    Value::Object(_) => self.generate_field(&fields, name, "Object"),
                };
            }
        }
        Schema::new(docs.len(), &vec![]);
    }

    fn generate_field(&self, fields: &BTreeMap<&str, Field>, name: &str, category: &str) {
        if fields.contains_key("name") {
            // Update the existing field.
        } else {
            // fields.insert(name, Field::new(name, 1, 1.0, false, &vec![]));
        }
    }
}
