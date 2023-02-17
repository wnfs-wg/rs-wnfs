use super::utils::error;
use crate::value;
use js_sys::{Object, Reflect};
use libipld::Ipld;
use wasm_bindgen::JsValue;
use wnfs::Metadata;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub(crate) struct JsMetadata<'a>(pub(crate) &'a Metadata);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl TryFrom<JsMetadata<'_>> for JsValue {
    type Error = js_sys::Error;

    fn try_from(value: JsMetadata<'_>) -> Result<Self, Self::Error> {
        let metadata = Object::new();

        if let Some(Ipld::Integer(i)) = value.0 .0.get("created") {
            Reflect::set(
                &metadata,
                &value!("created"),
                &value!(i64::try_from(*i)
                    .map_err(error("Cannot convert 'created' metadata value"))?
                    as f64),
            )?;
        }

        if let Some(Ipld::Integer(i)) = value.0 .0.get("modified") {
            Reflect::set(
                &metadata,
                &value!("modified"),
                &value!(i64::try_from(*i)
                    .map_err(error("Cannot convert 'modified' metadata value"))?
                    as f64),
            )?;
        }

        Ok(value!(metadata))
    }
}
