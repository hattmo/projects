use std::fmt::Display;

use super::{ConstPointer, ConstValue};

#[allow(non_camel_case_types)]
pub struct Class_info {
    pub(crate) name_index: ConstPointer<Utf8_info>,
}

impl Display for Class_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .name_index
            .resolve()
            .map(|v| format!("Class - {v}"))
            .unwrap_or_else(|e| format!("Class - {e}"));
        write!(f, "{out}")
    }
}
impl ConstValue<Class_info> {
    pub fn get_name_index(&self) -> Result<ConstValue<Utf8_info>, &'static str> {
        self.inner.borrow().1.name_index.resolve()
    }
    pub fn set_name_index(&self, name_index: &ConstPointer<Utf8_info>) {
        self.inner.borrow_mut().1.name_index = name_index.clone();
    }
}

#[allow(non_camel_case_types)]
pub struct Utf8_info {
    bytes: Vec<u8>,
}

impl Display for Utf8_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Utf8 - \"{}\"", String::from_utf8_lossy(&self.bytes))
    }
}

impl ConstValue<Utf8_info> {
    pub fn get_bytes(&self) -> Vec<u8> {
        self.inner.borrow().1.bytes.clone()
    }
    pub fn set_bytes(&self, bytes: impl AsRef<[u8]>) {
        self.inner.borrow_mut().1.bytes = bytes.as_ref().to_vec();
    }

    pub fn get_string(&self) -> String {
        String::from_utf8_lossy(&self.inner.borrow().1.bytes).to_string()
    }
}

#[allow(non_camel_case_types)]
pub struct Fieldref_info {
    class_index: ConstValue<Class_info>,
    name_and_type_index: ConstValue<NameAndType_info>,
}

impl Display for Fieldref_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fieldref - {} {}.{}",
            self.name_and_type_index.get_descriptor_index().get_string(),
            self.class_index.get_name_index().get_string(),
            self.name_and_type_index.get_name_index().get_string()
        )
    }
}

impl ConstValue<Fieldref_info> {
    pub fn get_class_index(&self) -> ConstValue<Class_info> {
        self.inner.borrow().1.class_index.clone()
    }
    pub fn set_class_index(&self, class_index: &ConstValue<Class_info>) {
        self.inner.borrow_mut().1.class_index = class_index.clone();
    }
    pub fn get_name_and_type_index(&self) -> ConstValue<NameAndType_info> {
        self.inner.borrow().1.name_and_type_index.clone()
    }
    pub fn set_name_and_type_index(&self, name_and_type_index: &ConstValue<NameAndType_info>) {
        self.inner.borrow_mut().1.name_and_type_index = name_and_type_index.clone();
    }
}

#[allow(non_camel_case_types)]
pub struct Methodref_info {
    class_index: ConstValue<Class_info>,
    name_and_type_index: ConstValue<NameAndType_info>,
}

impl Display for Methodref_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Methodref - {}.{}({})",
            self.class_index.get_name_index().get_string(),
            self.name_and_type_index.get_name_index().get_string(),
            self.name_and_type_index.get_descriptor_index().get_string()
        )
    }
}

impl ConstValue<Methodref_info> {
    pub fn get_class_index(&self) -> ConstValue<Class_info> {
        self.inner.borrow().1.class_index.clone()
    }
    pub fn set_class_index(&self, class_index: &ConstValue<Class_info>) {
        self.inner.borrow_mut().1.class_index = class_index.clone();
    }
    pub fn get_name_and_type_index(&self) -> ConstValue<NameAndType_info> {
        self.inner.borrow().1.name_and_type_index.clone()
    }
    pub fn set_name_and_type_index(&self, name_and_type_index: &ConstValue<NameAndType_info>) {
        self.inner.borrow_mut().1.name_and_type_index = name_and_type_index.clone();
    }
}

#[allow(non_camel_case_types)]
pub struct InterfaceMethodref_info {
    class_index: ConstValue<Class_info>,
    name_and_type_index: ConstValue<NameAndType_info>,
}

impl Display for InterfaceMethodref_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InterfaceMethodref - {}.{}({})",
            self.class_index.get_name_index().get_string(),
            self.name_and_type_index.get_name_index().get_string(),
            self.name_and_type_index.get_descriptor_index().get_string()
        )
    }
}

impl ConstValue<InterfaceMethodref_info> {
    pub fn get_class_index(&self) -> ConstValue<Class_info> {
        self.inner.borrow().1.class_index.clone()
    }
    pub fn set_class_index(&self, class_index: &ConstValue<Class_info>) {
        self.inner.borrow_mut().1.class_index = class_index.clone();
    }
    pub fn get_name_and_type_index(&self) -> ConstValue<NameAndType_info> {
        self.inner.borrow().1.name_and_type_index.clone()
    }
    pub fn set_name_and_type_index(&self, name_and_type_index: &ConstValue<NameAndType_info>) {
        self.inner.borrow_mut().1.name_and_type_index = name_and_type_index.clone();
    }
}

#[allow(non_camel_case_types)]
pub struct String_info {
    string_index: ConstValue<Utf8_info>,
}

impl Display for String_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"String - {}\"", self.string_index.get_string())
    }
}

impl ConstValue<String_info> {
    pub fn get_string_index(&self) -> ConstValue<Utf8_info> {
        self.inner.borrow().1.string_index.clone()
    }
    pub fn set_string_index(&self, string_index: &ConstValue<Utf8_info>) {
        self.inner.borrow_mut().1.string_index = string_index.clone();
    }
}

#[allow(non_camel_case_types)]
pub struct Integer_info {
    bytes: u32,
}

impl Display for Integer_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"Integer - {}\"", self.bytes)
    }
}

impl ConstValue<Integer_info> {
    pub fn get_bytes(&self) -> u32 {
        self.inner.borrow().1.bytes
    }
    pub fn set_bytes(&self, bytes: u32) {
        self.inner.borrow_mut().1.bytes = bytes;
    }
}

#[allow(non_camel_case_types)]
pub struct Float_info {
    bytes: f32,
}

impl Display for Float_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"Float - {}\"", self.bytes)
    }
}

impl ConstValue<Float_info> {
    pub fn get_bytes(&self) -> f32 {
        self.inner.borrow().1.bytes
    }
    pub fn set_bytes(&self, bytes: f32) {
        self.inner.borrow_mut().1.bytes = bytes;
    }
}

#[allow(non_camel_case_types)]
pub struct Long_info {
    bytes: u64,
}

impl Display for Long_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"Long - {}\"", self.bytes)
    }
}

impl ConstValue<Long_info> {
    pub fn get_bytes(&self) -> u64 {
        self.inner.borrow().1.bytes
    }
    pub fn set_bytes(&self, bytes: u64) {
        self.inner.borrow_mut().1.bytes = bytes;
    }
}

#[allow(non_camel_case_types)]
pub struct Double_info {
    bytes: f64,
}

impl Display for Double_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"Double - {}\"", self.bytes)
    }
}

impl ConstValue<Double_info> {
    pub fn get_bytes(&self) -> f64 {
        self.inner.borrow().1.bytes
    }
    pub fn set_bytes(&self, bytes: f64) {
        self.inner.borrow_mut().1.bytes = bytes;
    }
}

#[allow(non_camel_case_types)]
pub struct NameAndType_info {
    name_index: ConstValue<Utf8_info>,
    descriptor_index: ConstValue<Utf8_info>,
}

impl Display for NameAndType_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"NameAndType - {}({})\"",
            self.name_index.get_string(),
            self.descriptor_index.get_string()
        )
    }
}

impl ConstValue<NameAndType_info> {
    pub fn get_name_index(&self) -> ConstValue<Utf8_info> {
        self.inner.borrow().1.name_index.clone()
    }
    pub fn set_name_index(&self, name_index: &ConstValue<Utf8_info>) {
        self.inner.borrow_mut().1.name_index = name_index.clone();
    }
    pub fn get_descriptor_index(&self) -> ConstValue<Utf8_info> {
        self.inner.borrow().1.descriptor_index.clone()
    }
    pub fn set_descriptor_index(&self, descriptor_index: &ConstValue<Utf8_info>) {
        self.inner.borrow_mut().1.descriptor_index = descriptor_index.clone();
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum MethodHandleType {
    Fieldref_info(ConstValue<Fieldref_info>),
    Methodref_info(ConstValue<Methodref_info>),
    InterfaceMethodref_info(ConstValue<InterfaceMethodref_info>),
}

//TODO Check the refence matches the kind
#[allow(non_camel_case_types)]

pub struct MethodHandle_info {
    reference_kind: u8,
    reference_index: MethodHandleType,
}

impl Display for MethodHandle_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let other = match self.reference_index {
            MethodHandleType::Fieldref_info(ref ref_index) => format!(
                "{} {}",
                ref_index
                    .get_name_and_type_index()
                    .get_descriptor_index()
                    .get_string(),
                ref_index
                    .get_name_and_type_index()
                    .get_name_index()
                    .get_string()
            ),
            MethodHandleType::Methodref_info(ref ref_index) => format!(
                "{}({})",
                ref_index
                    .get_name_and_type_index()
                    .get_name_index()
                    .get_string(),
                ref_index
                    .get_name_and_type_index()
                    .get_descriptor_index()
                    .get_string(),
            ),
            MethodHandleType::InterfaceMethodref_info(ref ref_index) => format!(
                "{}({})",
                ref_index
                    .get_name_and_type_index()
                    .get_name_index()
                    .get_string(),
                ref_index
                    .get_name_and_type_index()
                    .get_descriptor_index()
                    .get_string(),
            ),
        };
        write!(
            f,
            "\"MethodHandle - kind:{} {}\"",
            self.reference_kind, other
        )
    }
}

impl ConstValue<MethodHandle_info> {
    pub fn get_reference_kind(&self) -> u8 {
        self.inner.borrow().1.reference_kind
    }
    pub fn set_reference_kind(&self, reference_kind: u8) {
        self.inner.borrow_mut().1.reference_kind = reference_kind;
    }
    pub fn get_reference_index(&self) -> MethodHandleType {
        self.inner.borrow().1.reference_index.clone()
    }
    pub fn set_reference_index(&self, reference_index: &MethodHandleType) {
        self.inner.borrow_mut().1.reference_index = reference_index.clone();
    }
}

#[allow(non_camel_case_types)]

pub struct MethodType_info {
    descriptor_index: ConstValue<Utf8_info>,
}

impl Display for MethodType_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"MethodType - {}\"", self.descriptor_index.get_string())
    }
}

impl ConstValue<MethodType_info> {
    pub fn get_descriptor_index(&self) -> ConstValue<Utf8_info> {
        self.inner.borrow().1.descriptor_index.clone()
    }
    pub fn set_descriptor_index(&self, descriptor_index: &ConstValue<Utf8_info>) {
        self.inner.borrow_mut().1.descriptor_index = descriptor_index.clone();
    }
}

#[allow(non_camel_case_types)]

pub struct InvokeDynamic_info {
    bootstrap_method_attr_index: u16, //This can be made smarter to point into the bootstrap method table
    name_and_type_index: ConstValue<NameAndType_info>,
}

impl Display for InvokeDynamic_info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"InvokeDynamic - index:{} {}({})\"",
            self.bootstrap_method_attr_index,
            self.name_and_type_index.get_name_index().get_string(),
            self.name_and_type_index.get_descriptor_index().get_string()
        )
    }
}

impl ConstValue<InvokeDynamic_info> {
    pub fn get_bootstrap_method_attr_index(&self) -> u16 {
        self.inner.borrow().1.bootstrap_method_attr_index
    }
    pub fn set_bootstrap_method_attr_index(&self, bootstrap_method_attr_index: u16) {
        self.inner.borrow_mut().1.bootstrap_method_attr_index = bootstrap_method_attr_index;
    }
    pub fn get_name_and_type_index(&self) -> ConstValue<NameAndType_info> {
        self.inner.borrow().1.name_and_type_index.clone()
    }
    pub fn set_name_and_type_index(&self, name_and_type_index: &ConstValue<NameAndType_info>) {
        self.inner.borrow_mut().1.name_and_type_index = name_and_type_index.clone();
    }
}
