mod fn_definition;

use fn_definition::FnDefinition;
use regex::Regex;

#[derive(Clone)]
pub struct WrapGen {
    functions: Vec<FnDefinition>,
    prefix: String,
    use_core: bool,
}

impl WrapGen {
    pub fn default() -> Self {
        WrapGen {
            functions: Vec::new(),
            prefix: String::from("rs_"),
            use_core: false,
        }
    }

    pub fn add_file(mut self, file: &str) -> Self {
        let mut fns = self.read_fns(std::fs::read_to_string(file).unwrap().as_str());
        self.functions.append(&mut fns);
        self
    }

    pub fn add_function(mut self, function: &str) -> Self {
        self.functions
            .push(FnDefinition::from_str(function).unwrap());
        self
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = String::from(prefix);
        self
    }

    pub fn use_core(mut self, use_core: bool) -> Self {
        self.use_core = use_core;
        self
    }

    pub fn new(file: &str) -> Self {
        WrapGen::default().add_file(file)
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

    pub fn generate(&self, outfile_path: &str) {
        let _ = std::fs::write(
            outfile_path,
            format!(
                "extern \"C\" {{
{}
}}

{}",
                self.functions
                    .iter()
                    .map(|v| format!("{}", v))
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
