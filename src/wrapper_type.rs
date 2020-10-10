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
pub struct WrapperType<'a> {
    pub original: &'a str,
    pub renamed: &'a str,
    fields: Vec<FieldType<'a>>,
}

impl<'a> WrapperType<'a> {
    pub fn new(original: &'a str, renamed: &'a str) -> Self {
        Self {
            original: original,
            renamed: renamed,
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::ReadWriteField(field_name, field_type));
        self
    }

    pub fn with_field_readonly(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::ReadOnlyField(field_name, field_type));
        self
    }

    pub fn with_field_writeonly(mut self, field_name: &'a str, field_type: &'a str) -> Self {
        self.fields
            .push(FieldType::WriteOnlyField(field_name, field_type));
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
