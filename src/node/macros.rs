#[macro_export]
macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name(pub u16);

        impl From<u16> for $name {
            fn from(value: u16) -> Self { Self(value) }
        }

        impl TryFrom<&str> for $name {
        type Error = std::num::ParseIntError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse::<u16>().map(Self)
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
#[macro_export]
macro_rules! node_impl {
    ($name:ident) => {
        impl NodeInput for $name {
            fn set_input(&mut self, key: NodeIOKey, value: NodeValue) {
                self.base.set_input(key, value);
            }
            fn get_input(&self, key: NodeIOKey) -> Option<&NodeValue> {
                self.base.get_input_value(key)
            }
            fn get_input_map(&self) -> &Vec<NodeIO> {
                &self.base.inputs
            }
        }
        impl NodeOutput for $name {
            fn get_output(&self, key: NodeIOKey) -> Option<&NodeValue> {
                self.base.get_output_value(key)
            }
            fn get_output_map(&self) -> &Vec<NodeIO> {
                &self.base.outputs
            }
            fn set_output(&mut self, key: NodeIOKey, value: NodeValue) {
                self.base.set_output(key, value)
            }
        }
        impl NodeParam for $name {
            fn set_param(&mut self, key: NodeIOKey, value: NodeValue) {
                self.base.set_param(key, value);
            }
            fn get_param(&self, key: NodeIOKey) -> Option<&NodeValue> {
                self.base.get_param(key).map_or(None, |p| p.value.as_ref())
            }
        }
    }
}
