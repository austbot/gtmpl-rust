use std::collections::HashMap;

use parse::{Tree, Parser, parse};
use funcs::Func;
use funcs::BUILTINS;
use node::TreeId;

/// The main template structure.
#[derive(Default)]
pub struct Template<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub funcs: Vec<&'a HashMap<String, Func>>,
    pub tree_ids: HashMap<TreeId, String>,
    pub tree_set: HashMap<String, Tree<'a>>,
}

impl<'a> Template<'a> {
    /// Creates a new empty template with a given `name`.
    pub fn with_name(name: &'a str) -> Template<'a> {
        Template {
            name: name,
            text: "",
            funcs: Vec::default(),
            tree_ids: HashMap::default(),
            tree_set: HashMap::default(),
        }
    }

    /// Adds custom functions to the template.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std::any::Any;
    /// use std::collections::HashMap;
    /// use std::sync::Arc;
    ///
    /// use gtmpl::{Context, Func, Value};
    ///
    /// fn hello_world(_args: &[Arc<Any>]) -> Result<Arc<Any>, String> {
    ///   Ok(Arc::new(Value::from("Hello World!")) as Arc<Any>)
    /// }
    ///
    /// let mut funcs = HashMap::new();
    /// funcs.insert(String::from("helloWorld"), hello_world as Func);
    /// let mut tmpl = gtmpl::Template::default();
    /// tmpl.add_funcs(&funcs);
    /// tmpl.parse("{{ helloWorld }}").unwrap();
    /// let output = tmpl.render(Context::empty());
    /// assert_eq!(&output.unwrap(), "Hello World!");
    /// ```
    pub fn add_funcs(&mut self, funcs: &'a HashMap<String, Func>) {
        self.funcs.push(funcs);
    }

    /// Parse the given `text` as template body.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut tmpl = gtmpl::Template::default();
    /// tmpl.parse("Hello World!").unwrap();
    /// ```
    pub fn parse(&mut self, text: &'a str) -> Result<(), String> {
        let mut funcs = vec![&BUILTINS as &HashMap<String, Func>];
        funcs.extend(&self.funcs);
        let parser = parse(self.name, text, funcs)?;
        match parser {
            Parser {
                funcs,
                tree_ids,
                tree_set,
                ..
            } => {
                self.funcs = funcs;
                self.tree_set = tree_set;
                self.tree_ids = tree_ids;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_mocked {
    use super::*;

    #[test]
    fn test_parse() {
        let mut t = Template::with_name("foo");
        assert!(t.parse(r#"{{ if eq "bar" "bar" }} 2000 {{ end }}"#).is_ok());
        assert!(t.tree_set.contains_key("foo"));
        assert!(t.tree_ids.contains_key(&1usize));
    }
}
