use serde::ser;

use crate::ser::{Error, Result, Serializer};

use super::MapKeySerializer;

pub struct SerializeMap<'a> {
    ser: &'a mut Serializer,
    first: bool,
}

impl<'a> SerializeMap<'a> {
    pub(crate) fn new(ser: &'a mut Serializer) -> Self {
        SerializeMap { ser, first: true }
    }
}

impl<'a> ser::SerializeMap for SerializeMap<'a> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok> {
        self.ser.buf.push(b'}');
        Ok(())
    }

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        if !self.first {
            self.ser.buf.push(b',');
        }
        self.first = false;
        // Use key serializer to unsure key type validity.
        key.serialize(MapKeySerializer { ser: self.ser })?;
        self.ser.buf.extend_from_slice(b":");
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }
}
