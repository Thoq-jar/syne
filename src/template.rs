use std::collections::HashMap;
use std::fs;
use regex::Regex;

pub struct Template {
    content: String,
}

const REGEX_ERROR: &str = "Failed to compile regex";

impl Template {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(Template { content })
    }

    pub fn from_string(content: &str) -> Self {
        Template {
            content: content.to_string(),
        }
    }

    pub fn render(&self, context: &HashMap<String, String>) -> String {
        let mut result = self.content.clone();

        let re = Regex::new(r"\{\{\s*(\w+)\s*}}").expect(REGEX_ERROR);

        result = re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = context.get(var_name) {
                value.clone()
            } else {
                caps[0].to_string()
            }
        }).to_string();

        result
    }

    pub fn render_with_loops(&self, context: &TemplateContext) -> String {
        let mut result = self.content.clone();

        result = self.process_loops(result, context);

        let re = Regex::new(r"\{\{\s*(\w+)\s*}}").expect(REGEX_ERROR);

        result = re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = context.variables.get(var_name) {
                value.clone()
            } else {
                caps[0].to_string()
            }
        }).to_string();

        result
    }

    fn process_loops(&self, mut content: String, context: &TemplateContext) -> String {
        let loop_re = Regex::new(r"(?s)\{%\s*for\s+(\w+)\s+in\s+(\w+)\s*%}(.*?)\{%\s*endfor\s*%}").expect(REGEX_ERROR);

        while let Some(captures) = loop_re.captures(&content) {
            let full_match = captures.get(0).expect("Failed to get a full match");
            let var_name = &captures[1];
            let list_name = &captures[2];
            let loop_body = &captures[3];

            let mut loop_result = String::new();

            if let Some(items) = context.lists.get(list_name) {
                for item in items.iter() {

                    let item_content = self.replace_loop_variable(loop_body, var_name, item);
                    loop_result.push_str(&item_content);
                }
            }

            content.replace_range(full_match.range(), &loop_result);
        }

        content
    }

    fn replace_loop_variable(&self, template: &str, var_name: &str, value: &str) -> String {
        let pattern = format!(r"\{{\{{\s*{}\s*\}}\}}", regex::escape(var_name));
        let re = Regex::new(&pattern).expect(REGEX_ERROR);
        re.replace_all(template, value).to_string()
    }
}

pub struct TemplateEngine {
    template_dir: String,
    cache: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new(template_dir: &str) -> Self {
        TemplateEngine {
            template_dir: template_dir.to_string(),
            cache: HashMap::new(),
        }
    }

    fn load_template_content(&mut self, name: &str) -> Result<String, std::io::Error> {
        if !self.cache.contains_key(name) {
            let template_path = format!("{}/{}.html", self.template_dir, name);
            let content = fs::read_to_string(&template_path)?;

            self.cache.insert(name.to_string(), content.clone());
            Ok(content)
        } else {
            Ok(self.cache.get(name).expect("Template wasn't found in the cache").clone())
        }
    }

    pub fn render(&mut self, template_name: &str, context: &TemplateContext) -> Result<String, std::io::Error> {
        let content = self.load_template_content(template_name)?;
        let template = Template { content };
        let result = template.render_with_loops(context);
        Ok(result)
    }

    pub fn render_simple(&mut self, template_name: &str, context: &HashMap<String, String>) -> Result<String, std::io::Error> {
        let content = self.load_template_content(template_name)?;
        let template = Template { content };
        let result = template.render(context);
        Ok(result)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

pub struct TemplateContext {
    pub variables: HashMap<String, String>,
    pub lists: HashMap<String, Vec<String>>,
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateContext {
    pub fn new() -> Self {
        TemplateContext {
            variables: HashMap::new(),
            lists: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn set_list(&mut self, key: &str, items: Vec<String>) {
        self.lists.insert(key.to_string(), items);
    }
}

#[macro_export]
macro_rules! template_context {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut context = $crate::template::TemplateContext::new();
            $(
                context.set($key, $value);
            )*
            context
        }
    };
}