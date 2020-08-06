mod fn_definition;
mod wrapper_type;

use fn_definition::FnDefinition;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone)]
pub struct WrapGen {
    functions: Vec<FnDefinition>,
    wrapped_types: HashMap<String, String>,
    prefix: String,
    use_core: bool,
}

impl WrapGen {
    /// Create a new `WrapGen` without reading from any files
    pub fn default() -> Self {
        WrapGen {
            functions: Vec::new(),
            wrapped_types: HashMap::new(),
            prefix: String::from("rs_"),
            use_core: false,
        }
    }

    /// Add all the functions from `file`
    pub fn add_file<P: AsRef<Path>>(mut self, file: P) -> Self {
        let mut fns = self.read_fns(std::fs::read_to_string(file).unwrap().as_str());
        self.functions.append(&mut fns);
        self
    }

    /// Add the single function from `function`.
    /// A semicolon at the end is optional here.
    pub fn add_function(mut self, function: &str) -> Self {
        self.functions
            .push(FnDefinition::from_str(function).unwrap());
        self
    }

    /// Set the prefix of the wrapped functions.
    /// Defaults to `rs_`
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = String::from(prefix);
        self
    }

    /// Determines if `core::ptr` or `std::ptr` should be used
    pub fn use_core(mut self, use_core: bool) -> Self {
        self.use_core = use_core;
        self
    }

    /// Create a new `WrapGen` and add the functions
    /// listed in `file`
    pub fn new<P: AsRef<Path>>(file: P) -> Self {
        WrapGen::default().add_file(file)
    }

    pub fn wrap_pointer_type(mut self, to_wrap: &str, wrapped_type: &str) -> Self {
        self.wrapped_types
            .insert(String::from(to_wrap), String::from(wrapped_type));
        self
    }

    fn create_wrapper(&self, to_wrap: &str) -> String {
        let wrapper_name = self
            .wrapped_types
            .get(to_wrap)
            .unwrap_or(&String::from(to_wrap))
            .clone();
        format!(
            "struct {} {{
    ptr: *mut {}
}}

impl {} {{
    fn from_ptr(ptr: *mut {}) -> Self {{
        Self {{ ptr }}
    }}

    fn get_ptr(&self) -> *mut {} {{
        self.ptr
    }}
}}",
            &wrapper_name, to_wrap, &wrapper_name, to_wrap, to_wrap
        )
    }

    fn read_fns(&self, lines: &str) -> Vec<FnDefinition> {
        let re =
            Regex::new(r"fn\s([a-z_0-9]+)\s?\(([a-z_:&*0-9,\s]*)\)\s?(\s->\s([a-z_:&*0-9\s]*))?;")
                .unwrap();

        let mut matches = Vec::new();

        for cap in re.captures_iter(lines) {
            matches.push(FnDefinition::from_cap(cap).unwrap());
        }

        matches
    }

    fn translate_function(&self, function: &FnDefinition) -> String {
        match &function.returns {
            Some(returns) => {
                if returns.ends_with("c_int") {
                    format!(
                        "fn {}{}({}) -> Result<(), {}> {{
    match unsafe {{ {}({}) }} {{
        0 => Ok(()),
        e => Err(e),
    }}
}}",
                        self.prefix,
                        function.get_name(),
                        function.get_params(),
                        function.get_returns(),
                        function.get_name(),
                        function.get_param_names().join(", ")
                    )
                } else {
                    format!(
                        "fn {}{}({}) -> Option<{}> {{
    let val = unsafe {{ {}({}) }};
    if val == {}::ptr::null_mut() {{
        return None;
    }} else {{
        return Some(val);
    }}
}}",
                        self.prefix,
                        function.get_name(),
                        function.get_params(),
                        function.get_returns(),
                        function.get_name(),
                        function.get_param_names().join(", "),
                        if self.use_core { "core" } else { "std" }
                    )
                }
            }
            None => format!(
                "fn {}{}({}) {{
    unsafe {{ {}({}) }};
}}",
                self.prefix,
                function.get_name(),
                function.get_params(),
                function.get_name(),
                function.get_param_names().join(", ")
            ),
        }
    }

    fn generate_extern_declarations(&self) -> String {
        if self.functions.is_empty() {
            String::new()
        } else {
            format!(
                "extern \"C\" {{
{}
}}",
                self.functions
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    }

    /// Generate wrappers for all previously added functions
    /// and write them to `outfile_path`
    pub fn generate<P: AsRef<Path>>(&self, outfile_path: P) {
        let _ = std::fs::write(
            outfile_path,
            format!(
                "{}

{}

{}",
                self.generate_extern_declarations(),
                self.wrapped_types
                    .keys()
                    .map(|k| self.create_wrapper(k))
                    .collect::<Vec<String>>()
                    .join("\n"),
                self.functions
                    .iter()
                    .map(|v| self.translate_function(v))
                    .collect::<Vec<String>>()
                    .join("\n\n")
            ),
        );
    }
}
