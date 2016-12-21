#![crate_name = "mongodb_schema"]

/// Represents a value type in a field.
///
/// # Fields
pub struct Type {
    pub name: String,
    pub count: i64,
    pub probability: f32,
    pub unique: i64
}

impl Type {

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
pub struct Field {
    pub name: String,
    pub count: i64,
    pub probability: f32,
    pub has_duplicates: bool,
    pub types: Vec<Type>
}

impl Field {

}

/// Represents a MongoDB schema.
///
/// # Fields
///
/// * `count` - The number of documents analysed.
/// * `fields` - The various fields in the schema.
pub struct Schema {
    pub count: i64,
    pub fields: Vec<Field>
}

/// The MongoDB Schema implementation.
///
/// Schemas themselves are immutable.
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
    pub fn new(count: i64, fields: Vec<Field>) -> Schema {
        Schema {
            count: count,
            fields: fields
        }
    }
}

/// Analyses documents to generate a schema.
pub struct SchemaAnalyser;

/// The analyser implementation.
impl SchemaAnalyser {
}
