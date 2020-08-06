#[derive(Clone)]
pub enum FieldType {
    ReadOnlyField(String, String),
    WriteOnlyField(String, String),
    ReadWriteField(String, String),
}

impl FieldType {
    fn generate_getter(name: &str, field_type: &str) -> String {
        format!(
            "fn get_{}(&self) -> {} {{
        unsafe {{ (*self.ptr).{} }}
    }}",
            name, field_type, name
        )
    }

    fn generate_setter(name: &str, field_type: &str) -> String {
        format!(
            "fn set_{}(&self, value: {}) {{
        unsafe {{ (*self.ptr).{} = value }};
    }}",
            name, field_type, name
        )
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
pub struct WrapperType {
    pub original: String,
    pub renamed: String,
    fields: Vec<FieldType>,
}

impl WrapperType {
    pub fn new(original: &str, renamed: &str) -> Self {
        Self {
            original: original.to_owned(),
            renamed: renamed.to_owned(),
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, field_name: &str, field_type: &str) -> Self {
        self.fields.push(FieldType::ReadWriteField(
            field_name.to_owned(),
            field_type.to_owned(),
        ));
        self
    }

    pub fn with_field_readonly(mut self, field_name: &str, field_type: &str) -> Self {
        self.fields.push(FieldType::ReadOnlyField(
            field_name.to_owned(),
            field_type.to_owned(),
        ));
        self
    }

    pub fn with_field_writeonly(mut self, field_name: &str, field_type: &str) -> Self {
        self.fields.push(FieldType::WriteOnlyField(
            field_name.to_owned(),
            field_type.to_owned(),
        ));
        self
    }

    pub fn generate(&self) -> String {
        format!(
            "struct {} {{
    ptr: *mut {}
}}

impl {} {{
    fn new(ptr: *mut {}) -> Self {{
        Self {{ ptr }}
    }}

    fn get_ptr(&self) -> *mut {} {{
        self.ptr
    }}

    {}
}}",
            self.renamed,
            self.original,
            self.renamed,
            self.original,
            self.original,
            self.fields
                .iter()
                .map(|f| f.generate())
                .collect::<Vec<String>>()
                .join("\n\n    ")
        )
    }
}
