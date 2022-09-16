use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};
pub mod const_types;
use const_types::*;
pub struct ConstPool {
    items: Vec<ConstantType>,
    next_index: u16,
}

impl ConstPool {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_index: 1,
        }
    }
    pub fn get_const(&self, index: u16) -> Option<ConstantType> {
        self.items.iter().find(|i| i.get_index() == index).cloned()
    }

    pub(crate) fn add_raw_const(&mut self, item: ConstantType) -> u16 {
        item.set_index(self.next_index);
        match item {
            ConstantType::Double_info(_) | ConstantType::Long_info(_) => {
                self.next_index += 2;
            }
            _ => {
                self.next_index += 1;
            }
        }
        self.items.push(item);
        self.next_index
    }

    pub fn add_class(&mut self, name_index: &ConstPointer<Utf8_info>) -> ConstValue<Class_info> {
        let out = ConstValue::new(
            self.next_index,
            Class_info {
                name_index: name_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Class_info(out.clone()));
        out
    }
    pub fn add_utf8(&mut self, bytes: impl AsRef<[u8]>) -> ConstValue<Utf8_info> {
        let out = ConstValue::new(
            self.next_index,
            Utf8_info {
                bytes: bytes.as_ref().to_vec(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Utf8_info(out.clone()));
        out
    }
    pub fn add_fieldref(
        &mut self,
        class_index: &ConstValue<Class_info>,
        name_and_type_index: &ConstValue<NameAndType_info>,
    ) -> ConstValue<Fieldref_info> {
        let out = ConstValue::new(
            self.next_index,
            Fieldref_info {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Fieldref_info(out.clone()));
        out
    }

    pub fn add_methodref(
        &mut self,
        class_index: &ConstValue<Class_info>,
        name_and_type_index: &ConstValue<NameAndType_info>,
    ) -> ConstValue<Methodref_info> {
        let out = ConstValue::new(
            self.next_index,
            Methodref_info {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Methodref_info(out.clone()));
        out
    }
    pub fn add_interface_methodref(
        &mut self,
        class_index: &ConstValue<Class_info>,
        name_and_type_index: &ConstValue<NameAndType_info>,
    ) -> ConstValue<InterfaceMethodref_info> {
        let out = ConstValue::new(
            self.next_index,
            InterfaceMethodref_info {
                class_index: class_index.clone(),
                name_and_type_index: name_and_type_index.clone(),
            },
        );
        self.next_index += 1;
        self.items
            .push(ConstantType::InterfaceMethodref_info(out.clone()));
        out
    }
    pub fn add_string(&mut self, string_index: &ConstValue<Utf8_info>) -> ConstValue<String_info> {
        let out = ConstValue::new(
            self.next_index,
            String_info {
                string_index: string_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::String_info(out.clone()));
        out
    }
    pub fn add_integer(&mut self, bytes: impl Into<u32>) -> ConstValue<Integer_info> {
        let out = ConstValue::new(
            self.next_index,
            Integer_info {
                bytes: bytes.into(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Integer_info(out.clone()));
        out
    }
    pub fn add_float(&mut self, bytes: impl Into<f32>) -> ConstValue<Float_info> {
        let out = ConstValue::new(
            self.next_index,
            Float_info {
                bytes: bytes.into(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::Float_info(out.clone()));
        out
    }
    pub fn add_long(&mut self, bytes: impl Into<u64>) -> ConstValue<Long_info> {
        let out = ConstValue::new(
            self.next_index,
            Long_info {
                bytes: bytes.into(),
            },
        );
        self.next_index += 2;
        self.items.push(ConstantType::Long_info(out.clone()));
        out
    }
    pub fn add_double(&mut self, bytes: impl Into<f64>) -> ConstValue<Double_info> {
        let out = ConstValue::new(
            self.next_index,
            Double_info {
                bytes: bytes.into(),
            },
        );
        self.next_index += 2;
        self.items.push(ConstantType::Double_info(out.clone()));
        out
    }
    pub fn add_name_and_type(
        &mut self,
        name_index: &ConstValue<Utf8_info>,
        descriptor_index: &ConstValue<Utf8_info>,
    ) -> ConstValue<NameAndType_info> {
        let out = ConstValue::new(
            self.next_index,
            NameAndType_info {
                name_index: name_index.clone(),
                descriptor_index: descriptor_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::NameAndType_info(out.clone()));
        out
    }
    pub fn add_method_handle(
        &mut self,
        reference_kind: u8,
        reference_index: &MethodHandleType,
    ) -> ConstValue<MethodHandle_info> {
        let out = ConstValue::new(
            self.next_index,
            MethodHandle_info {
                reference_kind,
                reference_index: reference_index.clone(),
            },
        );
        self.next_index += 1;
        self.items
            .push(ConstantType::MethodHandle_info(out.clone()));
        out
    }
    pub fn add_method_type(
        &mut self,
        descriptor_index: &ConstValue<Utf8_info>,
    ) -> ConstValue<MethodType_info> {
        let out = ConstValue::new(
            self.next_index,
            MethodType_info {
                descriptor_index: descriptor_index.clone(),
            },
        );
        self.next_index += 1;
        self.items.push(ConstantType::MethodType_info(out.clone()));
        out
    }
    pub fn add_invoke_dynamic(
        &mut self,
        bootstrap_method_attr_index: u16,
        name_and_type_index: &ConstValue<NameAndType_info>,
    ) -> ConstValue<InvokeDynamic_info> {
        let out = ConstValue::new(
            self.next_index,
            InvokeDynamic_info {
                bootstrap_method_attr_index,
                name_and_type_index: name_and_type_index.clone(),
            },
        );
        self.next_index += 1;
        self.items
            .push(ConstantType::InvokeDynamic_info(out.clone()));
        out
    }
}

impl Default for ConstPool {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ConstPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = self
            .items
            .iter()
            .map(|item| item.to_string())
            .intersperse("\n".to_string())
            .collect();
        write!(f, "{out}")
    }
}

type ConstEntry<T> = RefCell<(u16, T)>;

pub struct ConstValue<T> {
    inner: Rc<ConstEntry<T>>,
}

impl<T> Clone for ConstValue<T> {
    fn clone(&self) -> Self {
        let inner = Rc::clone(&self.inner);
        Self { inner }
    }
}

impl<T> ConstValue<T> {
    pub(crate) fn new(index: u16, inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new((index, inner))),
        }
    }
    pub fn get_index(&self) -> u16 {
        self.inner.borrow().0
    }
}

impl<T> Display for ConstValue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.borrow();
        write!(f, "#{}: {}", inner.0, inner.1)
    }
}

struct ResolvedConstPointer<T> {
    inner: Weak<ConstEntry<T>>,
}

impl<T> Clone for ResolvedConstPointer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Weak::clone(&self.inner),
        }
    }
}

impl<T> From<ConstValue<T>> for ResolvedConstPointer<T> {
    fn from(value: ConstValue<T>) -> Self {
        Self {
            inner: Rc::downgrade(&value.inner),
        }
    }
}

enum ConstPointer<T> {
    Resolved(ResolvedConstPointer<T>),
    Unresolved(u16),
    Unset,
}

impl<T> Clone for ConstPointer<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Resolved(resolved) => Self::Resolved(resolved.clone()),
            Self::Unresolved(index) => Self::Unresolved(*index),
            Self::Unset => Self::Unset,
        }
    }
}

impl<T> ConstPointer<T> {
    fn resolve(&self) -> Result<ConstValue<T>, &'static str> {
        match self {
            Self::Resolved(resolved) => resolved
                .inner
                .upgrade()
                .ok_or("ConstValue has been dropped")
                .map(|inner| ConstValue { inner }),
            Self::Unresolved(_) => Err("ConstPointer is unresolved"),
            Self::Unset => Err("ConstPointer is unset"),
        }
    }
}

impl<T> TryFrom<ConstPointer<T>> for ConstValue<T> {
    type Error = &'static str;
    fn try_from(value: ConstPointer<T>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum ConstantType {
    Class_info(ConstValue<Class_info>),
    Fieldref_info(ConstValue<Fieldref_info>),
    Methodref_info(ConstValue<Methodref_info>),
    InterfaceMethodref_info(ConstValue<InterfaceMethodref_info>),
    String_info(ConstValue<String_info>),
    Integer_info(ConstValue<Integer_info>),
    Float_info(ConstValue<Float_info>),
    Long_info(ConstValue<Long_info>),
    Double_info(ConstValue<Double_info>),
    NameAndType_info(ConstValue<NameAndType_info>),
    Utf8_info(ConstValue<Utf8_info>),
    MethodHandle_info(ConstValue<MethodHandle_info>),
    MethodType_info(ConstValue<MethodType_info>),
    InvokeDynamic_info(ConstValue<InvokeDynamic_info>),
}

impl ConstantType {
    fn get_index(&self) -> u16 {
        match self {
            Self::Class_info(c) => c.inner.borrow().0,
            Self::Fieldref_info(c) => c.inner.borrow().0,
            Self::Methodref_info(c) => c.inner.borrow().0,
            Self::InterfaceMethodref_info(c) => c.inner.borrow().0,
            Self::String_info(c) => c.inner.borrow().0,
            Self::Integer_info(c) => c.inner.borrow().0,
            Self::Float_info(c) => c.inner.borrow().0,
            Self::Long_info(c) => c.inner.borrow().0,
            Self::Double_info(c) => c.inner.borrow().0,
            Self::NameAndType_info(c) => c.inner.borrow().0,
            Self::Utf8_info(c) => c.inner.borrow().0,
            Self::MethodHandle_info(c) => c.inner.borrow().0,
            Self::MethodType_info(c) => c.inner.borrow().0,
            Self::InvokeDynamic_info(c) => c.inner.borrow().0,
        }
    }
    fn set_index(&self, index: u16) {
        match self {
            Self::Class_info(c) => c.inner.borrow_mut().0 = index,
            Self::Fieldref_info(c) => c.inner.borrow_mut().0 = index,
            Self::Methodref_info(c) => c.inner.borrow_mut().0 = index,
            Self::InterfaceMethodref_info(c) => c.inner.borrow_mut().0 = index,
            Self::String_info(c) => c.inner.borrow_mut().0 = index,
            Self::Integer_info(c) => c.inner.borrow_mut().0 = index,
            Self::Float_info(c) => c.inner.borrow_mut().0 = index,
            Self::Long_info(c) => c.inner.borrow_mut().0 = index,
            Self::Double_info(c) => c.inner.borrow_mut().0 = index,
            Self::NameAndType_info(c) => c.inner.borrow_mut().0 = index,
            Self::Utf8_info(c) => c.inner.borrow_mut().0 = index,
            Self::MethodHandle_info(c) => c.inner.borrow_mut().0 = index,
            Self::MethodType_info(c) => c.inner.borrow_mut().0 = index,
            Self::InvokeDynamic_info(c) => c.inner.borrow_mut().0 = index,
        }
    }
}

impl Display for ConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class_info(c) => write!(f, "{c}"),
            Self::Fieldref_info(c) => write!(f, "{c}"),
            Self::Methodref_info(c) => write!(f, "{c}"),
            Self::InterfaceMethodref_info(c) => write!(f, "{c}"),
            Self::String_info(c) => write!(f, "{c}"),
            Self::Integer_info(c) => write!(f, "{c}"),
            Self::Float_info(c) => write!(f, "{c}"),
            Self::Long_info(c) => write!(f, "{c}"),
            Self::Double_info(c) => write!(f, "{c}"),
            Self::NameAndType_info(c) => write!(f, "{c}"),
            Self::Utf8_info(c) => write!(f, "{c}"),
            Self::MethodHandle_info(c) => write!(f, "{c}"),
            Self::MethodType_info(c) => write!(f, "{c}"),
            Self::InvokeDynamic_info(c) => write!(f, "{c}"),
        }
    }
}
