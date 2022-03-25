use std::collections::HashMap;

pub struct Scope {
    map_stack: Vec<HashMap<String, isize>>,
}

impl Scope {
    pub fn new() -> Self {
        let map = HashMap::new();
        let map_stack = vec![map];
        Scope { map_stack }
    }

    pub fn set_var(&mut self, var_name: &str, value: isize) {
        let mut current_map = self.map_stack.pop().unwrap();
        current_map.insert(var_name.into(), value);
        self.map_stack.push(current_map);
    }

    /// Get `var_name`'s last defined value.
    pub fn get_var_value(&mut self, var_name: &str) -> Option<isize> {
        for map in self.map_stack.iter().rev() {
            if let Some(value) = map.get(var_name.into()) {
                return Some(*value);
            }
        }
        None
    }

    pub fn drop(&mut self) {
        self.map_stack.pop();
    }
    pub fn add(&mut self) {
        self.map_stack.push(HashMap::new());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_scope() {
        let mut scope = Scope::new();
        // This would be equivalent to
        // var pepe = 12;
        // var i = 5;
        // si(i > 0){
        //   var pepe = 23;
        //   pepe = 49;
        // }
        scope.set_var("pepe", 12);
        scope.add();
        scope.set_var("pepe", 23);
        let value = scope.get_var_value("pepe");
        assert_eq!(value, Some(23));
        scope.set_var("pepe", 49);
        let value = scope.get_var_value("pepe");
        assert_eq!(value, Some(49));
        scope.drop();
        let value = scope.get_var_value("pepe");
        assert_eq!(value, Some(12));
    }
}
