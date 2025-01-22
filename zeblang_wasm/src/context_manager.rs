use std::collections::HashMap;

pub struct Context {
    vars: HashMap<String, i32>,
}

pub struct ContextManager {
    contexts: HashMap<String, Context>,
}

impl Context {
    fn new() -> Self {
        return Self {
            vars: HashMap::new(),
        };
    }
}

impl ContextManager {
    pub fn new() -> Self {
        return Self {
            contexts: HashMap::new(),
        };
    }

    pub fn get_context(&mut self, context: &str, var: &str) -> Result<&i32, String> {
        return self
            .contexts
            .get(context)
            .ok_or("Context Not Found".to_string())?
            .vars
            .get(var)
            .ok_or("Var not found in current context".to_string());
    }

    pub fn new_context(&mut self, name: &str) {
        self.contexts.insert(name.to_owned(), Context::new());
    }

    // pub fn insert_var(&mut self, context: &str, name: &str, value: i32) -> Result<(), String> {
    //     self.contexts
    //         .get(context)
    //         .ok_or("Context Not Found")?
    //         .vars
    //         .insert(name.to_owned(), value);
    //     Ok(())
    // }
}
