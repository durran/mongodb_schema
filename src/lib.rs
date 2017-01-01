#![crate_name = "schema"]
#![crate_type = "lib"]

/// Represents a value type in a field.
///
/// # Fields
///
/// * `name` - The name of the BSON type.
/// * `count` - The number of occurences for the field.
/// * `probability` - The probability in the current field.
/// * `unique` - The count of unique values.
pub struct Type<'t> {
    pub name: &'t str,
    pub count: i64,
    pub probability: f32,
    pub unique: i64
}

/// The Type implementation.
impl<'t> Type<'t> {

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
    /// assert_eq!(schema::Type::new("Decimal128", 5, 0.75, 26).name, "Decimal128");
    /// ```
    pub fn new(name: &'t str, count: i64, probability: f32, unique: i64) -> Type {
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
    pub name: &'f str,
    pub count: i64,
    pub probability: f32,
    pub has_duplicates: bool,
    pub types: &'f Vec<Type<'f>>
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
        name: &'f str,
        count: i64,
        probability: f32,
        has_duplicates: bool,
        types: &'f Vec<Type<'f>>) -> Field<'f> {

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
    pub count: i64,
    pub fields: &'s Vec<Field<'s>>
}

/// The MongoDB Schema implementation.
///
/// Schemas themselves are immutable.
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
    pub fn new(count: i64, fields: &'s Vec<Field<'s>>) -> Schema {
        Schema {
            count: count,
            fields: fields
        }
    }
}

/// Analyses documents to generate a schema.
pub struct Analyser;

/// The analyser implementation.
impl Analyser {
}
