extern crate serde_json;

use std::collections::BTreeMap;
use serde_json::Value;

/// The null type constant.
static NULL: &'static str = "Null";

/// The boolean type constant.
static BOOLEAN: &'static str = "Boolean";

/// The i64 type constant.
static INT64: &'static str = "Int64";

/// The u64 type constant.
static UINT64: &'static str = "UInt64";

/// The f64 type constant.
static DOUBLE: &'static str = "Double";

/// The string type constant.
static STRING: &'static str = "String";

/// The array type constant.
static ARRAY: &'static str = "Array";

/// The object type constant.
static OBJECT: &'static str = "Object";

/// Represents a value type in a field.
///
/// # Fields
///
/// * `name` - The name of the BSON type.
/// * `count` - The number of occurences for the field.
/// * `probability` - The probability in the current field.
/// * `unique` - The count of unique values.
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub count: i64,
    pub probability: f32,
    pub has_duplicates: bool,
    pub types: Vec<Type>
}

/// The field implementation.
impl Field {

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
        types: Vec<Type>) -> Field {

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
pub struct Schema {
    pub count: usize,
    pub fields: Vec<Field>
}

/// The MongoDB Schema implementation.
impl Schema {

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
    pub fn new(count: usize, fields: Vec<Field>) -> Schema {
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
    pub fn run(&self) -> Schema {
        let mut fields = BTreeMap::new();
        let docs = self.documents.as_array().unwrap();
        for document in docs {
            let doc = document.as_object().unwrap();
            for (name, value) in doc.iter() {
                match *value {
                    Value::Null => self.analyse_field(&mut fields, name, NULL),
                    Value::Bool(_) => self.analyse_field(&mut fields, name, BOOLEAN),
                    Value::I64(_) => self.analyse_field(&mut fields, name, INT64),
                    Value::U64(_) => self.analyse_field(&mut fields, name, UINT64),
                    Value::F64(_) => self.analyse_field(&mut fields, name, DOUBLE),
                    Value::String(_) => self.analyse_field(&mut fields, name, STRING),
                    Value::Array(_) => self.analyse_field(&mut fields, name, ARRAY),
                    Value::Object(_) => self.analyse_field(&mut fields, name, OBJECT),
                };
            }
        }
        return Schema::new(docs.len(), fields.values().cloned().collect());
    }

    /// Analyse a field and add it to the map of fields.
    ///
    /// # Paramters
    ///
    /// * `fields` - The tree map of fields.
    /// * `name` - The name of the field.
    /// * `category` - The field type.
    fn analyse_field(&self, fields: &mut BTreeMap<String, Field>, name: &str, category: &str) {
        if fields.contains_key(name) {
            // Update the existing field.
        } else {
            let mut types = Vec::new();
            types.push(Type::new(category.to_string(), 1, 1.0, 1));
            fields.insert(name.to_string(), Field::new(name.to_string(), 1, 1.0, false, types));
        }
    }
}
