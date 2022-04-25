use std::borrow::Cow;

/// Details for a parking space may be added to the `<object>` element.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct ParkingSpace {
    /// Access definitions for the parking space. Parking spaces tagged with "women" and
    /// "handicapped" are vehicles of type car.
    pub access: Access,
    /// Free text, depending on application
    pub restrictions: Option<String>,
}

impl ParkingSpace {
    pub fn visit_attributes(
        &self,
        visitor: impl for<'b> FnOnce(
            Cow<'b, [xml::attribute::Attribute<'b>]>,
        ) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_attributes_flatten!(
            visitor,
            "access" => Some(self.access.as_str()),
            "restrictions" => self.restrictions.as_deref(),
        )
    }

    pub fn visit_children(
        &self,
        mut visitor: impl FnMut(xml::writer::XmlEvent) -> xml::writer::Result<()>,
    ) -> xml::writer::Result<()> {
        visit_children!(visitor);
        Ok(())
    }
}
impl<'a, I> TryFrom<crate::parser::ReadContext<'a, I>> for ParkingSpace
where
    I: Iterator<Item = xml::reader::Result<xml::reader::XmlEvent>>,
{
    type Error = crate::parser::Error;

    fn try_from(read: crate::parser::ReadContext<'a, I>) -> Result<Self, Self::Error> {
        Ok(Self {
            access: read.attribute("access")?,
            restrictions: read.attribute_opt("restrictions")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub enum Access {
    All,
    Car,
    Women,
    Handicapped,
    Bus,
    Truck,
    Electric,
    Residents,
}

impl_from_str_as_str!(
    Access,
    "all" => All,
    "car" => Car,
    "women" => Women,
    "handicapped" => Handicapped,
    "bus" => Bus,
    "truck" => Truck,
    "electric" => Electric,
    "residents" => Residents,
);
