use std::collections::HashMap;

pub trait IodineObject<'a> {
    fn get_attributes(&self) -> &'a AttributeDictionary;
    fn get_attributes_mut(&mut self) -> &'a mut AttributeDictionary;
}

pub trait IodineType {
    fn get_name(&self) -> String;
}

pub type AttributeDictionary<'a> = HashMap<String, Box<&'a dyn IodineObject<'a>>>;

pub enum IodineTypes<'a> {
    TypeDefinition { attribs: AttributeDictionary<'a> },
}

impl<'a> IodineType for IodineTypes<'a> {
    fn get_name(&self) -> String {
        match self {
            IodineTypes::TypeDefinition { attribs: _ } => String::from("TypeDef"),
        }
    }
}

impl<'a> IodineObject<'a> for IodineTypes<'a> {
    fn get_attributes(&self) -> &'a AttributeDictionary {
        match self {
            IodineTypes::TypeDefinition { attribs } => attribs,
        }
    }

    fn get_attributes_mut(&mut self) -> &'a mut AttributeDictionary {
        match self {
            IodineTypes::TypeDefinition { attribs } => attribs,
        }
    }
}
