use super::{metadata::JsMetadata, PrivateDirectory, PrivateForest, PublicDirectory};
use crate::{fs::JsResult, value};
use js_sys::{Array, Error, Object, Reflect};
use std::{fmt::Debug, rc::Rc};
use wasm_bindgen::JsValue;
use wnfs::{
    common::Metadata,
    private::{
        forest::hamt::HamtForest as WnfsHamtForest, PrivateDirectory as WnfsPrivateDirectory,
    },
    public::PublicDirectory as WnfsPublicDirectory,
};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn error<E>(message: &str) -> impl FnOnce(E) -> Error + '_
where
    E: Debug,
{
    move |e| Error::new(&format!("{message}: {e:?}"))
}

pub(crate) fn anyhow_error<E>(message: &str) -> impl FnOnce(E) -> anyhow::Error + '_
where
    E: Debug,
{
    move |e| anyhow::Error::msg(format!("{message}: {e:?}"))
}

pub(crate) fn map_to_rust_vec<T, F: FnMut(JsValue) -> JsResult<T>>(
    array: &Array,
    f: F,
) -> JsResult<Vec<T>> {
    array
        .to_vec()
        .into_iter()
        .map(f)
        .collect::<JsResult<Vec<_>>>()
}

pub(crate) fn convert_path_segments(path_segments: &Array) -> JsResult<Vec<String>> {
    map_to_rust_vec(path_segments, |v| {
        v.as_string()
            .ok_or_else(|| Error::new("Invalid path segments: Expected an array of strings"))
    })
}

pub(crate) fn create_public_op_result<T: Into<JsValue>>(
    root_dir: Rc<WnfsPublicDirectory>,
    result: T,
) -> JsResult<JsValue> {
    let op_result = Object::new();

    Reflect::set(
        &op_result,
        &value!("rootDir"),
        &PublicDirectory(root_dir).into(),
    )
    .map_err(error("Failed to set rootDir"))?;
    Reflect::set(&op_result, &value!("result"), &result.into())
        .map_err(error("Failed to set result"))?;

    Ok(value!(op_result))
}

pub(crate) fn create_private_op_result<T: Into<JsValue>>(
    root_dir: Rc<WnfsPrivateDirectory>,
    forest: Rc<WnfsHamtForest>,
    result: T,
) -> JsResult<JsValue> {
    let op_result = Array::new();

    Reflect::set(
        &op_result,
        &value!("rootDir"),
        &PrivateDirectory(root_dir).into(),
    )
    .map_err(error("Failed to set rootDir"))?;
    Reflect::set(&op_result, &value!("forest"), &PrivateForest(forest).into())
        .map_err(error("Failed to set forest"))?;
    Reflect::set(&op_result, &value!("result"), &result.into())
        .map_err(error("Failed to set result"))?;

    Ok(value!(op_result))
}

pub(crate) fn create_private_forest_result(
    result: JsValue,
    forest: Rc<WnfsHamtForest>,
) -> JsResult<JsValue> {
    let op_result = Array::new();

    Reflect::set(&op_result, &value!(0), &result).map_err(error("Failed to set result"))?;
    Reflect::set(&op_result, &value!(1), &PrivateForest(forest).into())
        .map_err(error("Failed to set forest"))?;

    Ok(value!(op_result))
}

pub(crate) fn create_ls_entry(name: &String, metadata: &Metadata) -> JsResult<JsValue> {
    let entry = Object::new();

    Reflect::set(&entry, &value!("name"), &value!(name)).map_err(error("Failed to set name"))?;
    Reflect::set(
        &entry,
        &value!("metadata"),
        &JsMetadata(metadata).try_into()?,
    )
    .map_err(error("Failed to set metadata"))?;

    Ok(value!(entry))
}

#[inline]
pub(crate) fn expect_bytes<const N: usize>(bytes: Vec<u8>) -> JsResult<[u8; N]> {
    bytes.try_into().map_err(|v: Vec<u8>| {
        Error::new(&format!(
            "Unexpected number of bytes received. Expected {N}, but got {}",
            v.len()
        ))
    })
}
