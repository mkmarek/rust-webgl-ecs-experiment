use crate::entity::*;
use crate::base::System;
use web_sys::{WebGlRenderingContext};

pub struct Application {
    pub document: web_sys::Document,
    pub context: WebGlRenderingContext,
    pub entities: Vec<Entity>,
    pub systems: Vec<Box<System>>
}

impl Application {
    pub fn remove_entity(&mut self, entity: &Entity) -> bool {
        let index = self.entities.iter().position(|x| (*x).id == entity.id);

        if index == None {
            return false;
        }

        self.entities.remove(index.unwrap());

        return true;
    }
    pub fn add_entity(&mut self, mut entity: Entity) {
        self.remove_entity(&entity);

        entity.id = self.entities.len().to_string();

        self.entities.push(entity);
    }
    pub fn get_with_components(&self, components: &str) -> Vec<&Entity> {
        let component_types: Vec<&str> = components.split(',').collect();
        let mut result = Vec::new();
        
        for entity in &self.entities {
            let mut contains_all = true;
            for component in &component_types {
                if !entity.contains_component(&component.to_string()) {
                    contains_all = false;
                }
            }

            if contains_all == true {
                result.push(entity);
            }
        }

        return result;
    }
}