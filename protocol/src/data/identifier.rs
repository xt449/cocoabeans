use std::fmt::{Display, Formatter};

pub struct Identifier<'a> {
    namespace: Option<&'a str>,
    key: &'a str
}

impl<'a> Identifier<'a> {
    pub fn new_minecraft(key: &'a str) -> Self {
        return Identifier {
            namespace: None,
            key: key,
        };
    }
    pub fn new_other(namespace: &'a str, key: &'a str) -> Self {
        return Identifier {
            namespace: Some(namespace),
            key: key,
        };
    }
}

impl<'a> Display for Identifier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}:{}", match self.namespace {
            None => {"minecraft"}
            Some(namespace) => {namespace}
        }, self.key));
    }
}
