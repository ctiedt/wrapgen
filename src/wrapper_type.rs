#[derive(Clone)]
pub enum FieldType<'a> {
    ReadOnlyField(&'a str, &'a str),
    WriteOnlyField(&'a str, &'a str),
    ReadWriteField(&'a str, &'a str),
}

impl<'a> FieldType<'a> {
    fn generate_getter(name: &'a str, field_type: &'a str) -> String {
        format!(
            "fn get_{}(&self) -> {} {{
        unsafe {{ (*self.ptr).{} }}
    }}",
            name, field_type, name
        )
    }

    fn generate_setter(name: &'a str, field_type: &'a str) -> String {
        format!(
            "fn set_{}(&self, value: {}) {{
        unsafe {{ (*self.ptr).{} = value }};
    }}",
            name, field_type, name
        )
    }

    fn generate_getter_signature(name: &'a str, field_type: &'a str) -> String {
        format!("fn get_{}(&self) -> {};", name, field_type)
    }

    fn generate_setter_signature(name: &'a str, field_type: &'a str) -> String {
        format!("fn set_{}(&self, value: {});", name, field_type)
    }

    fn generate_signatures(&self) -> String {
        match self {
            Self::ReadOnlyField(name, field_type) => {
                Self::generate_getter_signature(name, field_type)
            }
            Self::WriteOnlyField(name, field_type) => {
                Self::generate_setter_signature(name, field_type)
            }
            Self::ReadWriteField(name, field_type) => format!(
                "{}\n    {}",
                Self::generate_getter_signature(name, field_type),
                Self::generate_setter_signature(name, field_type)
            ),
        }
    }

    fn generate(&self) -> String {
        match self {
            Self::ReadOnlyField(name, field_type) => Self::generate_getter(name, field_type),
            Self::WriteOnlyField(name, field_type) => Self::generate_setter(name, field_type),
            Self::ReadWriteField(name, field_type) => format!(
                "{}\n\n    {}",
                Self::generate_getter(name, field_type),
                Self::generate_setter(name, field_type)
            ),
        }
    }
}

#[derive(Clone)]
/// The representation of a pointer type to wrap.
/// If you wrap a pointer type `*mut T`, it will be represtented
/// as `Wrapper<T>` where `Wrapper` is defined as the following
/// zero-cost abstraction:
///
///```
///pub struct Wrapper<T> {
///    ptr: *mut T
///}
///
///impl<T> Wrapper<T> {
///    pub fn from_ptr(ptr: *mut T) -> Self {
///        Self { ptr }
///    }
///
///    pub fn get_ptr(&self) -> *mut T {
///        self.ptr
///    }
///}
/// ```
///
/// WrapGen can create methods to safely access fields present on `*mut T`.
///
/// # Example
///
/// ```
///WrapperType::new("inode", "Inode")
///    .with_field("i_sb", "*mut super_block")
///    .with_field_writeonly("i_ino", "cty::c_int")
/// ```
pub struct WrapperType<'a> {
    pub original: &'a str,
    pub renamed: &'a str,
    fields: Vec<FieldType<'a>>,
}

impl<'a> WrapperType<'a> {
    /// Create a new wrapper for type `*mut original`
    pub fn new(original: &'a str, renamed: &'a str) -> Self {
        Self {
            original,
            renamed,
            fields: Vec::new(),
        }
    }

    /// Add a field present on `*mut original` that has a getter and setter
    pub fn with_field(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::ReadWriteField(field_name, field_type));
        self
    }

    /// Add a field present on `*mut original` that will only be read
    pub fn with_field_readonly(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::ReadOnlyField(field_name, field_type));
        self
    }

    /// Add a field present on `*mut original` that will only be written to
    pub fn with_field_writeonly(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::WriteOnlyField(field_name, field_type));
        self
    }

    /// Generate the implementation of the wrapper
    pub fn generate(&self) -> String {
        format!(
            "type {} = Wrapper<{}>;

pub trait {}Ops {{
    {}
}}

impl {}Ops for {} {{
    {}
}}",
            self.renamed,
            self.original,
            self.renamed,
            self.fields
                .iter()
                .map(|f| f.generate_signatures())
                .collect::<Vec<String>>()
                .join("\n    "),
            self.renamed,
            self.renamed,
            self.fields
                .iter()
                .map(|f| f.generate())
                .collect::<Vec<String>>()
                .join("\n\n    "),
        )
    }
}
