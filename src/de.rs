//! Deserialization support for the `application/x-www-form-urlencoded` format.

use serde::de;
use serde::de::value::MapDeserializer;
use std::borrow::Cow;
use std::marker::PhantomData;
use url::form_urlencoded::Parse;

pub use serde::de::value::Error;

/// A deserializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level outputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Main `deserialize` methods defers to `deserialize_map`.
///
/// * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
///   defers to `deserialize`.
pub struct Deserializer<'a, T> {
    deserializer: MapDeserializer<Parse<'a>, Cow<'a, str>, Cow<'a, str>, Error>,
    marker: PhantomData<T>,
}

impl<'a, T> de::Deserializer for Deserializer<'a, T>
{
    type Error = Error;

    fn deserialize<V>(
            &mut self, visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(
            &mut self, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        visitor.visit_map(&mut self.deserializer)
    }

    fn deserialize_seq<V>(
            &mut self, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        visitor.visit_seq(&mut self.deserializer)
    }

    fn deserialize_seq_fixed_size<V>(
            &mut self, _len: usize, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor
    {
        visitor.visit_seq(&mut self.deserializer)
    }

    forward_to_deserialize! {
        bool
        usize
        u8
        u16
        u32
        u64
        isize
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        unit
        option
        bytes
        unit_struct
        newtype_struct
        tuple_struct
        struct
        struct_field
        tuple
        enum
        ignored_any
    }
}