////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

use crate::js::perspective::JsPerspectiveViewConfig;
use crate::utils::*;

use super::aggregates::*;
use super::filters::*;
use super::sort::*;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(test)]
use {crate::*, js_sys::Array};

#[cfg(test)]
use wasm_bindgen_test::*;

#[derive(Clone, Deserialize, Default, Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(deny_unknown_fields)]
pub struct ViewConfig {
    #[serde(default)]
    pub row_pivots: Vec<String>,

    #[serde(default)]
    pub column_pivots: Vec<String>,

    #[serde(default)]
    pub columns: Vec<Option<String>>,

    #[serde(default)]
    pub filter: Vec<Filter>,

    #[serde(default)]
    pub sort: Vec<Sort>,

    #[serde(default)]
    pub expressions: Vec<String>,

    #[serde(default)]
    pub aggregates: HashMap<String, Aggregate>,
}

impl ViewConfig {
    fn _apply<T>(field: &mut T, update: Option<T>) -> bool {
        match update {
            None => false,
            Some(update) => {
                *field = update;
                true
            }
        }
    }

    pub fn as_jsvalue(&self) -> Result<JsPerspectiveViewConfig, JsValue> {
        let mut new_config = self.clone();
        new_config.columns.retain(|x| x.is_some());
        JsValue::from_serde(&new_config)
            .into_jserror()
            .map(|x| x.unchecked_into())
    }

    pub fn is_pivot(&self) -> bool {
        !self.row_pivots.is_empty() || !self.column_pivots.is_empty()
    }

    pub fn reset(&mut self) {
        std::mem::swap(self, &mut ViewConfig::default());
    }

    /// Apply `ViewConfigUpdate` to a `ViewConfig`, ignoring any fields in `update`
    /// which were unset.
    pub fn apply_update(&mut self, update: ViewConfigUpdate) -> bool {
        let mut changed = false;
        changed = Self::_apply(&mut self.row_pivots, update.row_pivots) || changed;
        changed =
            Self::_apply(&mut self.column_pivots, update.column_pivots) || changed;
        changed = Self::_apply(&mut self.columns, update.columns) || changed;
        changed = Self::_apply(&mut self.filter, update.filter) || changed;
        changed = Self::_apply(&mut self.sort, update.sort) || changed;
        changed = Self::_apply(&mut self.aggregates, update.aggregates) || changed;
        changed = Self::_apply(&mut self.expressions, update.expressions) || changed;
        changed
    }

    pub fn is_column_expression_in_use(&self, name: &str) -> bool {
        let name = name.to_owned();
        self.row_pivots.contains(&name)
            || self.column_pivots.contains(&name)
            || self.sort.iter().any(|x| x.0 == name)
            || self.filter.iter().any(|x| x.0 == name)
            || self.columns.contains(&Some(name))
    }
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ViewConfigUpdate {
    #[serde(default)]
    pub row_pivots: Option<Vec<String>>,

    #[serde(default)]
    pub column_pivots: Option<Vec<String>>,

    #[serde(default)]
    pub columns: Option<Vec<Option<String>>>,

    #[serde(default)]
    pub filter: Option<Vec<Filter>>,

    #[serde(default)]
    pub sort: Option<Vec<Sort>>,

    #[serde(default)]
    pub expressions: Option<Vec<String>>,

    #[serde(default)]
    pub aggregates: Option<HashMap<String, Aggregate>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test]
    pub fn test_multiaggregate_weighted_mean() {
        let x = js_object!(
            "aggregates",
            js_object!(
                "x",
                [JsValue::from("weighted mean"), JsValue::from("y")]
                    .iter()
                    .collect::<Array>()
            )
        );

        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(
            *rec.aggregates.get("x").unwrap(),
            Aggregate::MultiAggregate(MultiAggregate::WeightedMean, "y".to_owned())
        );
    }

    #[wasm_bindgen_test]
    pub fn test_row_pivots() {
        let x = js_object!(
            "row_pivots",
            [JsValue::from("Test")].iter().collect::<Array>()
        );
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(rec.row_pivots, vec!("Test"));
    }

    #[wasm_bindgen_test]
    pub fn test_column_pivots() {
        let x = js_object!(
            "column_pivots",
            [JsValue::from("Test")].iter().collect::<Array>()
        );
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(rec.column_pivots, vec!("Test"));
    }

    #[wasm_bindgen_test]
    pub fn test_column_filters() {
        let filter = ["Test", "contains", "aaa"]
            .iter()
            .map(|x| JsValue::from(*x))
            .collect::<Array>();

        let x = js_object!("filter", [filter].iter().collect::<Array>());
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(
            rec.filter,
            vec!(Filter(
                "Test".to_owned(),
                FilterOp::Contains,
                FilterTerm::Scalar(Scalar::String("aaa".to_owned()))
            ))
        );
    }

    #[wasm_bindgen_test]
    pub fn test_column_filters_operator_and_num_arg() {
        let filter = [JsValue::from("Test"), JsValue::from("<"), JsValue::from(4)]
            .iter()
            .collect::<Array>();

        let x = js_object!("filter", [filter].iter().collect::<Array>());
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(
            rec.filter,
            vec!(Filter(
                "Test".to_owned(),
                FilterOp::LT,
                FilterTerm::Scalar(Scalar::Float(4_f64))
            ))
        );
    }

    #[wasm_bindgen_test]
    pub fn test_column_sorts() {
        let sort = ["Test", "asc"]
            .iter()
            .map(|x| JsValue::from(*x))
            .collect::<Array>();

        let x = js_object!("sort", [sort].iter().collect::<Array>());
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(rec.sort, vec!(Sort("Test".to_owned(), SortDir::Asc,)));
    }

    #[wasm_bindgen_test]
    pub fn test_column_sorts_multiple() {
        let sort1 = ["Test1", "asc"]
            .iter()
            .map(|x| JsValue::from(*x))
            .collect::<Array>();

        let sort2 = ["Test2", "desc"]
            .iter()
            .map(|x| JsValue::from(*x))
            .collect::<Array>();

        let x = js_object!("sort", [sort1, sort2].iter().collect::<Array>());
        let rec: ViewConfig = x.into_serde().unwrap();
        assert_eq!(
            rec.sort,
            vec!(
                Sort("Test1".to_owned(), SortDir::Asc),
                Sort("Test2".to_owned(), SortDir::Desc)
            )
        );
    }

    #[wasm_bindgen_test]
    pub fn test_apply_update_empty_returns_changed_false() {
        let mut view_config = ViewConfig::default();
        let update = ViewConfigUpdate::default();
        let changed = view_config.apply_update(update);
        assert!(!changed);
        assert_eq!(view_config, ViewConfig::default());
    }

    #[wasm_bindgen_test]
    pub fn test_apply_update_row_pivots() {
        let mut view_config = ViewConfig::default();
        let update = ViewConfigUpdate {
            row_pivots: Some(vec!["Test".to_owned()]),
            ..ViewConfigUpdate::default()
        };

        assert_eq!(view_config.row_pivots.len(), 0);
        let changed = view_config.apply_update(update);
        assert!(changed);
        assert_eq!(view_config.row_pivots, vec!("Test".to_owned()));
    }

    #[wasm_bindgen_test]
    pub fn test_apply_update_row_pivots_then_column_pivots() {
        let mut view_config = ViewConfig::default();
        let update = ViewConfigUpdate {
            row_pivots: Some(vec!["Test".to_owned()]),
            ..ViewConfigUpdate::default()
        };

        assert_eq!(view_config.row_pivots.len(), 0);
        let changed = view_config.apply_update(update);
        assert!(changed);
        assert_eq!(view_config.row_pivots, vec!("Test".to_owned()));
        assert_eq!(view_config.column_pivots.len(), 0);
        let update = ViewConfigUpdate {
            column_pivots: Some(vec!["Test2".to_owned()]),
            ..ViewConfigUpdate::default()
        };

        let changed = view_config.apply_update(update);
        assert!(changed);
        assert_eq!(view_config.row_pivots, vec!("Test".to_owned()));
        assert_eq!(view_config.column_pivots, vec!("Test2".to_owned()));
    }
}
