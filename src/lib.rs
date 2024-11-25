#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use roaring::bitmap::RoaringBitmap;

#[napi(js_name = "RoaringBitmap")]
pub struct JsRoaringBitmap {
    inner: RoaringBitmap,
}

#[napi]
impl JsRoaringBitmap {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
           inner: RoaringBitmap::new(),
        }
    }

    #[napi]
    pub fn insert(&mut self, value: u32) {
        self.inner.insert(value);
    }

    #[napi]
    pub fn remove(&mut self, value: u32) {
        self.inner.remove(value);
    }

    #[napi]
    pub fn is_disjoint(&self, other: &JsRoaringBitmap) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    #[napi]
    pub fn is_subset(&self, other: &JsRoaringBitmap) -> bool {
        self.inner.is_subset(&other.inner)
    }

    #[napi]
    pub fn is_superset(&self, other: &JsRoaringBitmap) -> bool {
        self.inner.is_superset(&other.inner)
    }

    #[napi]
    pub fn full() -> Self {
        Self {
            inner: RoaringBitmap::full(),
        }
    }

    #[napi]
    pub fn container(&self, value: u32) -> bool {
        self.inner.contains(value)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.len() as u32
    }
}
