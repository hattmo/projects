use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub fn create_htmx_form(data: &impl Serialize) -> String {
    let mut serializer = HTMXSerializer {
        data: String::new(),
    };
    data.serialize(&mut serializer).unwrap();
    serializer.data
}

struct HTMXSerializer {
    data: String,
}

impl<'a> SerializeSeq for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_seq_element");
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_seq_end");
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_tuple_element");
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_tuple_end");
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_tuple_struct_field");
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_tuple_struct_end");
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_tuple_variant_field");
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_tuple_variant_end");
        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_map_key");
        key.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_map_value");
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_map_end");
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_struct_field: {}", key);
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_struct_end");
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_struct_variant_field: {}", key);
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_struct_variant_end");
        Ok(())
    }
}

impl<'a> Serializer for &'a mut HTMXSerializer {
    type Ok = ();

    type Error = HTMXSerializerError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        println!("serialize_bool: {}", v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        println!("serialize_i8: {}", v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        println!("serialize_i16: {}", v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        println!("serialize_i32: {}", v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        println!("serialize_i64: {}", v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        println!("serialize_u8: {}", v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        println!("serialize_u16: {}", v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        println!("serialize_u32: {}", v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        println!("serialize_u64: {}", v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        println!("serialize_f32: {}", v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        println!("serialize_f64: {}", v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        println!("serialize_char: {}", v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        println!("serialize_str: {}", v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        println!("serialize_bytes: {:?}", v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_none");
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_some");
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        println!("serialize_unit");
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        println!("serialize_unit_struct: {}", name);
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        println!(
            "serialize_unit_variant: {} ({}:{})",
            name, variant_index, variant
        );
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        println!("serialize_newtype_struct: {}", name);
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        println!(
            "serialize_newtype_variant {} ({}:{})",
            name, variant_index, variant
        );
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        println!("serialize_seq ({:?})", len);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        println!("serialize_tuple ({})", len);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        println!("serialize_tuple_struct: {} ({})", name, len);
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        println!(
            "serialize_tuple_variant: {} ({}:{}) ({})",
            name, variant_index, variant, len
        );
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        println!("serialize_map ({:?})", len);
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        println!("serialize_struct: {} ({})", name, len);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        println!(
            "serialize_struct_variant: {} ({}:{}) ({})",
            name, variant_index, variant, len
        );
        Ok(self)
    }
}

#[derive(Debug)]
enum HTMXSerializerError {
    Done,
}

impl Display for HTMXSerializerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Im a serializer error")
    }
}

impl Error for HTMXSerializerError {}

impl serde::ser::Error for HTMXSerializerError {
    fn custom<T: Display>(_msg: T) -> Self {
        HTMXSerializerError::Done
    }
}

#[cfg(test)]
mod test {
    use super::super::ServerSettings;
    use super::*;

    #[derive(Serialize)]
    enum Foo {
        BAR { one: u32, two: u8 },
        BAZ,
        BUZZ(u32),
    }
    #[test]
    fn test() {
        let settings = Foo::BAR { one: 3, two: 3 };
        let form = create_htmx_form(&settings);
        println!("{}", form);
    }
}
