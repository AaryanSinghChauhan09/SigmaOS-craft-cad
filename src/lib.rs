#[derive(Debug, Clone)]
pub struct Parameter { pub name: String, pub value: f64 }

#[derive(Debug, Default)]
pub struct CadDocument { parameters: Vec<Parameter> }
impl CadDocument {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, name: impl Into<String>, val: f64) {
        self.parameters.push(Parameter { name: name.into(), value: val });
    }
    pub fn get(&self, name: &str) -> Option<f64> {
        self.parameters.iter().find(|p| p.name == name).map(|p| p.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_param() {
        let mut d = CadDocument::new();
        d.add("radius", 5.0);
        assert_eq!(d.get("radius"), Some(5.0));
    }
}
