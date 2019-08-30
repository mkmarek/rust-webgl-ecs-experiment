use crate::base::*;

pub struct Entity {
    pub id: String,
    components: Vec<Box<Component>>,
}

impl Entity {
    pub fn new() -> Entity {
        let components: Vec<Box<Component>> = Vec::new();
        return Entity { components: components, id: String::from("") }
    }

    pub fn contains_component(&self, name: &String) -> bool {
        for entity in &self.components {
            if entity.is(name) {
                return true;
            }
        }

        return false;
    }
    pub fn remove_component(&mut self, name: &str) -> bool {
        let index = self.components.iter().position(|x| (*x).is(name));

        if index == None {
            return false;
        }

        self.components.remove(index.unwrap());

        return true;
    }
    pub fn get_component(&self, name: &str) -> Option<&Box<dyn Component>> {
        let index = self.components.iter().position(|x| (*x).is(name));

        if index == None {
            return None;
        }

        return self.components.get(index.unwrap());
    }
    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.remove_component(&component.get_name());

        self.components.push(Box::new(component));
    }
}