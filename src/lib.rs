#![crate_name = "mongodb_schema"]
#![crate_type = "lib"]

/// Represents a value type in a field.
///
/// # Fields
///
/// * `name` - The name of the BSON type.
/// * `count` - The number of occurences for the field.
/// * `probability` - The probability in the current field.
/// * `unique` - The count of unique values.
pub struct Type<'schema> {
    pub name: &'schema str,
    pub count: i64,
    pub probability: f32,
    pub unique: i64
}

/// The Type implementation.
impl<'schema> Type<'schema> {

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
    pub fn new(name: &str, count: i64, probability: f32, unique: i64) -> Type {
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
pub struct Field<'schema> {
    pub name: &'schema str,
    pub count: i64,
    pub probability: f32,
    pub has_duplicates: bool,
    pub types: Vec<Type<'schema>>
}

impl<'schema> Field<'schema> {

}

/// Represents a MongoDB schema.
///
/// # Fields
///
/// * `count` - The number of documents analysed.
/// * `fields` - The various fields in the schema.
pub struct Schema<'schema> {
    pub count: i64,
    pub fields: Vec<Field<'schema>>
}

/// The MongoDB Schema implementation.
///
/// Schemas themselves are immutable.
impl<'schema> Schema<'schema> {

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
