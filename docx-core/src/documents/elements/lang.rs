use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Lang {
    pub val: String,
}

impl Lang {
    pub fn new(val: impl Into<String>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for Lang {
    fn build_to<W: std::io::Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .lang(&self.val)?
            .into_inner()
    }
}
