use crate::junction::{ContactPoint, ElementDir};
use crate::road::geometry::PlanView;
use crate::road::lane::Lanes;
use crate::road::profile::{ElevationProfile, LateralProfile};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uom::si::f64::Length;
use uom::si::length::meter;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub mod geometry;
pub mod lane;
pub mod profile;

/// In ASAM OpenDRIVE, the road network is represented by `<road>` elements. Each road runs along
/// one road reference line. A road shall have at least one lane with a width larger than 0.
/// Vehicles may drive in both directions of the reference line. The standard driving direction is
/// defined by the value which is assigned to the @rule attribute (RHT=right-hand traffic,
/// LHT=left-hand traffic).
/// ASAM OpenDRIVE roads may be roads in the real road network or artificial road network created
/// for application use. Each road is described by one or more `<road>` elements. One `<road>`
/// element may cover a long stretch of a road, shorter stretches between junctions, or even several
/// roads. A new `<road>` element should only start if the properties of the road cannot be
/// described within the previous `<road>` element or if a junction is required.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Road {
    /// Unique ID within the database. If it represents an integer number, it should comply to
    /// `uint32_t` and stay within the given range.
    pub id: String,
    /// ID of the junction to which the road belongs as a connecting road (= -1 for none)
    pub junction: String,
    /// Total length of the reference line in the xy-plane. Change in length due to elevation is not
    /// considered.
    /// Only positive values are valid.
    pub length: Length,
    /// Name of the road. May be chosen freely.
    pub name: Option<String>,
    /// Basic rule for using the road; RHT=right-hand traffic, LHT=left-hand traffic. When this
    /// attribute is missing, RHT is assumed.
    pub rule: Option<Rule>,
    pub link: Option<Link>,
    #[serde(rename = "planView")]
    pub plan_view: PlanView,
    #[serde(rename = "elevationProfile")]
    pub elevation_profile: Option<ElevationProfile>,
    #[serde(rename = "lateralProfile")]
    pub lateral_profile: Option<LateralProfile>,
    pub lanes: Lanes,
    // pub objects: (),
    // pub signals: (),
    // pub surface: (),
    // pub raildroad: (),
}
impl Road {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut link = None;
        let mut plan_view = None;
        let mut elevation_profile = None;
        let mut lateral_profile = None;
        let mut lanes = None;

        find_map_parse_elem!(
            events,
            "link" => |attributes| {
                link = Some(Link::from_events(events, attributes)?);
                Ok(())
            },
            "planView" true => |attributes| {
                plan_view = Some(PlanView::from_events(events, attributes)?);
                Ok(())
            },
            "elevationProfile" => |attributes| {
                elevation_profile = Some(ElevationProfile::from_events(events, attributes)?);
                Ok(())
            },
            "lateralProfile" => |attributes| {
                lateral_profile = Some(LateralProfile::from_events(events, attributes)?);
                Ok(())
            },
            "lanes" true => |attributes| {
                lanes = Some(Lanes::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(Self {
            id: find_map_parse_attr!(attributes, "id", String)?,
            junction: find_map_parse_attr!(attributes, "junction", String)?,
            length: find_map_parse_attr!(attributes, "length", f64).map(Length::new::<meter>)?,
            name: find_map_parse_attr!(attributes, "name", Option<String>)?,
            rule: find_map_parse_attr!(attributes, "rule", Option<Rule>)?,
            link,
            plan_view: plan_view.unwrap(),
            elevation_profile,
            lateral_profile,
            lanes: lanes.unwrap(),
        })
    }
}

/// Follows the road header if the road is linked to a successor or a predecessor. Isolated roads
/// may omit this element.
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    pub predecessor: Option<PredecessorSuccessor>,
    pub successor: Option<PredecessorSuccessor>,
    // TODO pub additional_data: Vec<AdditionalData>,
}
impl Link {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        _attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        let mut this = Self::default();

        find_map_parse_elem!(
            events,
            "predecessor" => |attributes| {
                this.predecessor = Some(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            },
            "successor" => |attributes| {
                this.successor = Some(PredecessorSuccessor::from_events(events, attributes)?);
                Ok(())
            }
        );

        Ok(this)
    }
}

/// Successors and predecessors can be junctions or roads. For each, different attribute sets shall
/// be used.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PredecessorSuccessor {
    /// Contact point of link on the linked element
    #[serde(rename = "contactPoint")]
    pub contact_point: Option<ContactPoint>,
    /// To be provided when elementS is used for the connection definition. Indicates the direction
    /// on the predecessor from which the road is entered.
    #[serde(rename = "elementDir")]
    pub element_dir: Option<ElementDir>,
    /// ID of the linked element
    #[serde(rename = "elementId")]
    pub element_id: String,
    /// Alternative to contactPoint for virtual junctions. Indicates a connection within the
    /// predecessor, meaning not at the start or end of the predecessor. Shall only be used for
    /// elementType "road"
    #[serde(rename = "elementS")]
    pub element_s: Option<Length>,
    /// Type of the linked element
    #[serde(rename = "elementType")]
    pub element_type: Option<ElementType>,
}

impl PredecessorSuccessor {
    pub fn from_events(
        events: &mut impl Iterator<Item = xml::reader::Result<XmlEvent>>,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<Self, crate::parser::Error> {
        find_map_parse_elem!(events);
        Ok(Self {
            contact_point: find_map_parse_attr!(attributes, "contactPoint", Option<ContactPoint>)?,
            element_dir: find_map_parse_attr!(attributes, "elementDir", Option<ElementDir>)?,
            element_id: find_map_parse_attr!(attributes, "elementId", String)?,
            element_s: find_map_parse_attr!(attributes, "elementS", Option<f64>)?
                .map(Length::new::<meter>),
            element_type: find_map_parse_attr!(attributes, "elementType", Option<ElementType>)?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ElementType {
    #[serde(rename = "road")]
    Road,
    #[serde(rename = "junction")]
    Junction,
}

impl FromStr for ElementType {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("road") => Ok(Self::Road),
            _ if s.eq_ignore_ascii_case("junction") => Ok(Self::Junction),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Rule {
    #[serde(rename = "RHT")]
    RightHandTraffic,
    #[serde(rename = "LHT")]
    LeftHandTraffic,
}

impl FromStr for Rule {
    type Err = crate::parser::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("LHT") => Ok(Self::LeftHandTraffic),
            _ if s.eq_ignore_ascii_case("RHT") => Ok(Self::RightHandTraffic),
            _ => Err(crate::parser::Error::invalid_value_for::<Self, _>(s)),
        }
    }
}
