mod fn_definition;
mod wrapper_type;

use fn_definition::FnDefinition;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
pub use wrapper_type::WrapperType;

#[derive(Clone)]
pub struct WrapGen<'a> {
    functions: Vec<FnDefinition<'a>>,
    wrapped_types: Vec<WrapperType<'a>>,
    prefix: &'a str,
    use_core: bool,
    _included_files: Vec<String>,
}

impl<'a> WrapGen<'a> {
    /// Create a new `WrapGen` without reading from any files
    pub fn default() -> Self {
        WrapGen {
            functions: Vec::new(),
            wrapped_types: Vec::new(),
            prefix: "rs_",
            use_core: false,
            _included_files: Vec::new(),
        }
    }

    /// Add all the functions from `file`
    pub fn add_file<P: AsRef<Path>>(mut self, file: P) -> Self {
        let lines = std::fs::read_to_string(file).unwrap();
        self._included_files.push(lines);
        self
    }

    /// Add the single function from `function`.
    /// A semicolon at the end is optional here.
    pub fn add_function(mut self, function: &'a str) -> Self {
        self.functions
            .push(FnDefinition::from_str(function).unwrap());
        self
    }

    /// Set the prefix of the wrapped functions.
    /// Defaults to `rs_`
    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = prefix;
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

    /// Will create a wrapper around a type (usually a pointer)
    /// to allow safe access to fields
    pub fn wrap_pointer_type(mut self, to_wrap: WrapperType<'a>) -> Self {
        self.wrapped_types.push(to_wrap);
        self
    }

    fn wrapped_types(&self) -> HashMap<&str, &str> {
        let mut types = HashMap::new();
        for (original, wrapper) in self
            .wrapped_types
            .iter()
            .map(|v| (v.original.clone(), v.renamed.clone()))
        {
            types.insert(original, wrapper);
        }
        types
    }

    fn read_fns(lines: &'a str) -> Vec<FnDefinition> {
        let re =
            Regex::new(r"fn\s([a-z_0-9]+)\s?\(([a-z_:&*0-9,\s]*)\)\s?(->\s([a-z_:&*0-9\s]*))?;")
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
        return Some({});
    }}
}}",
                        self.prefix,
                        function.get_name(),
                        function.get_params(),
                        self.wrapped_types()
                            .get(function.get_returns().split_terminator(" ").nth(1).unwrap())
                            .unwrap_or(&function.get_returns()),
                        function.get_name(),
                        function.get_param_names().join(", "),
                        if self.use_core { "core" } else { "std" },
                        if let Some(wrapper) = self
                            .wrapped_types()
                            .get(function.get_returns().split_terminator(" ").nth(1).unwrap())
                        {
                            format!("{}::new(val)", wrapper)
                        } else {
                            "val".to_owned()
                        }
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
                    .map(|v| format!("    {}", v))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    }

    /// Generate wrappers for all previously added functions
    /// and write them to `outfile_path`
    pub fn generate<P: AsRef<Path>>(&'a mut self, outfile_path: P) {
        for lines in &self._included_files {
            self.functions.append(&mut Self::read_fns(&lines))
        }
        let _ = std::fs::write(
            outfile_path,
            format!(
                "{}

{}

{}",
                self.generate_extern_declarations(),
                self.wrapped_types
                    .iter()
                    .map(|v| v.generate())
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
