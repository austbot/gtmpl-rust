use std::collections::HashMap;

use parse::{Tree, Parser, parse};
use funcs::Func;
use funcs::BUILTINS;
use node::TreeId;

pub struct Template<'a> {
    name: &'a str,
    text: &'a str,
    funcs: Vec<&'a HashMap<String, Func>>,
    tree_ids: HashMap<TreeId, String>,
    tree_set: HashMap<String, Tree<'a>>,
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str) -> Template<'a> {
        Template {
            name: name,
            text: "",
            funcs: Vec::default(),
            tree_ids: HashMap::default(),
            tree_set: HashMap::default(),
        }
    }

    pub fn parse(&mut self, text: &'a str) -> Result<(),String> {
        let funcs = vec!(&BUILTINS as &HashMap<String, Func>);
        let parser = parse(self.name, text, funcs)?;
        match parser {
            Parser { funcs, tree_ids, tree_set, .. } => {
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
        let mut t = Template::new("foo");
        assert!(t.parse(r#"{{ if eq "bar" "bar" }} 2000 {{ end }}"#).is_ok());
        assert!(t.tree_set.contains_key("foo"));

    }
}
