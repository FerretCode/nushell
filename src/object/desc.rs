use crate::object::types::Type;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum DescriptorName {
    String(String),
    ValueOf,
}

impl DescriptorName {
    crate fn display(&self) -> &str {
        match self {
            DescriptorName::String(s) => s,
            DescriptorName::ValueOf => "value",
        }
    }

    crate fn as_string(&self) -> Option<&str> {
        match self {
            DescriptorName::String(s) => Some(s),
            DescriptorName::ValueOf => None,
        }
    }

    crate fn is_string(&self, string: &str) -> bool {
        match self {
            DescriptorName::String(s) => s == string,
            DescriptorName::ValueOf => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, new)]
pub struct DataDescriptor {
    crate name: DescriptorName,
    crate readonly: bool,
    crate ty: Type,
}

impl From<&str> for DataDescriptor {
    fn from(input: &str) -> DataDescriptor {
        DataDescriptor {
            name: DescriptorName::String(input.to_string()),
            readonly: true,
            ty: Type::Any,
        }
    }
}

impl From<String> for DataDescriptor {
    fn from(input: String) -> DataDescriptor {
        DataDescriptor {
            name: DescriptorName::String(input),
            readonly: true,
            ty: Type::Any,
        }
    }
}

impl DescriptorName {
    crate fn for_string_name(name: impl Into<String>) -> DescriptorName {
        DescriptorName::String(name.into())
    }
}

impl DataDescriptor {
    crate fn value_of() -> DataDescriptor {
        DataDescriptor {
            name: DescriptorName::ValueOf,
            readonly: true,
            ty: Type::Any,
        }
    }

    crate fn for_name(name: impl Into<DescriptorName>) -> DataDescriptor {
        DataDescriptor {
            name: name.into(),
            readonly: true,
            ty: Type::Any,
        }
    }

    crate fn for_string_name(name: impl Into<String>) -> DataDescriptor {
        DataDescriptor::for_name(DescriptorName::for_string_name(name))
    }

    crate fn copy(&self) -> DataDescriptor {
        self.clone()
    }
}
