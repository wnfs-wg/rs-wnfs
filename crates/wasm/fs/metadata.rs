use std::collections::BTreeMap;

use crate::fs::JsResult;
use crate::value;
use js_sys::{Object, Reflect};
use libipld::Ipld;
use wasm_bindgen::JsValue;
use wnfs::Metadata;

use super::utils::error;

pub(crate) struct JsMetadata<'a>(pub(crate) &'a Metadata);

impl TryFrom<JsMetadata<'_>> for JsValue {
    type Error = js_sys::Error;

    fn try_from(value: JsMetadata<'_>) -> Result<Self, Self::Error> {
        let metadata = Object::new();
        if let Some(unix_fs_meta) = value.0.get_unix_fs() {
            Reflect::set(
                &metadata,
                &value!("unixFsMeta"),
                &convert_unix_fs_meta(unix_fs_meta)?,
            )?;
        }

        Ok(value!(metadata))
    }
}

fn convert_unix_fs_meta(unix_fs_meta: &BTreeMap<String, Ipld>) -> JsResult<JsValue> {
    let obj = Object::new();

    if let Some(Ipld::Integer(i)) = unix_fs_meta.get("created") {
        Reflect::set(
            &obj,
            &value!("created"),
            &value!(i64::try_from(*i).map_err(error("Cannot convert created value"))?),
        )?;
    }

    if let Some(Ipld::Integer(i)) = unix_fs_meta.get("modified") {
        Reflect::set(
            &obj,
            &value!("modified"),
            &value!(i64::try_from(*i).map_err(error("Cannot convert modified value"))?),
        )?;
    }

    if let Some(Ipld::Integer(i)) = unix_fs_meta.get("mode") {
        Reflect::set(
            &obj,
            &value!("mode"),
            &value!(u32::try_from(*i).map_err(error("Cannot convert mode value"))?),
        )?;
    }

    if let Some(Ipld::String(s)) = unix_fs_meta.get("kind") {
        Reflect::set(&obj, &value!("kind"), &value!(s))?;
    }

    Ok(value!(obj))
}
