////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

use derivative::Derivative;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

/// A mutable reference for capturing the `ComponentLink<_>` props arguments, a useful
/// function for tests.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Default(bound = ""))]
pub struct WeakComponentLink<C: Component>(Rc<RefCell<Option<ComponentLink<C>>>>);

impl<C: Component> Deref for WeakComponentLink<C> {
    type Target = Rc<RefCell<Option<ComponentLink<C>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: Component> PartialEq for WeakComponentLink<C> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
