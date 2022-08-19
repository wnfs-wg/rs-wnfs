use std::{fmt::Debug, rc::Rc};

use crate::{fs::JsResult, value};
use js_sys::{Array, Error, Object, Reflect};
use wasm_bindgen::JsValue;
use wnfs::{
    private::{PrivateDirectory as WnfsPrivateDirectory, PrivateForest as WnfsPrivateForest},
    public::PublicDirectory as WnfsPublicDirectory,
};

use super::{PrivateDirectory, PrivateForest, PublicDirectory};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

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
    )?;
    Reflect::set(&op_result, &value!("result"), &result.into())?;

    Ok(value!(op_result))
}

pub(crate) fn create_private_op_result<T: Into<JsValue>>(
    root_dir: Rc<WnfsPrivateDirectory>,
    hamt: Rc<WnfsPrivateForest>,
    result: T,
) -> JsResult<JsValue> {
    let op_result = Object::new();

    Reflect::set(
        &op_result,
        &value!("rootDir"),
        &PrivateDirectory(root_dir).into(),
    )?;
    Reflect::set(&op_result, &value!("hamt"), &PrivateForest(hamt).into())?;
    Reflect::set(&op_result, &value!("result"), &result.into())?;

    Ok(value!(op_result))
}

pub fn error<E>(message: &str) -> impl FnOnce(E) -> js_sys::Error + '_
where
    E: Debug,
{
    move |e| Error::new(&format!("{message}: {e:?}"))
}
