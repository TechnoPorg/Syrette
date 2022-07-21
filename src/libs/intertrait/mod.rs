/**
 * Originally from Intertrait by CodeChain
 *
 * <https://github.com/CodeChain-io/intertrait>
 * <https://crates.io/crates/intertrait/0.2.2>
 *
 * Licensed under either of
 *
 * Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

 * at your option.
*/
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use linkme::distributed_slice;
use once_cell::sync::Lazy;

mod hasher;

use hasher::BuildFastHasher;

pub mod cast_box;
pub mod cast_rc;

pub type BoxedCaster = Box<dyn Any + Send + Sync>;

#[distributed_slice]
pub static CASTERS: [fn() -> (TypeId, BoxedCaster)] = [..];

static CASTER_MAP: Lazy<HashMap<(TypeId, TypeId), BoxedCaster, BuildFastHasher>> =
    Lazy::new(|| {
        CASTERS
            .iter()
            .map(|f| {
                let (type_id, caster) = f();
                ((type_id, (*caster).type_id()), caster)
            })
            .collect()
    });

pub struct Caster<T: ?Sized + 'static>
{
    /// Casts a `Box` holding a trait object for `Any` to another `Box` holding a trait object
    /// for trait `T`.
    pub cast_box: fn(from: Box<dyn Any>) -> Box<T>,

    /// Casts an `Rc` holding a trait object for `Any` to another `Rc` holding a trait object
    /// for trait `T`.
    pub cast_rc: fn(from: Rc<dyn Any>) -> Rc<T>,
}

impl<T: ?Sized + 'static> Caster<T>
{
    pub fn new(
        cast_box: fn(from: Box<dyn Any>) -> Box<T>,
        cast_rc: fn(from: Rc<dyn Any>) -> Rc<T>,
    ) -> Caster<T>
    {
        Caster::<T> { cast_box, cast_rc }
    }
}

/// Returns a `Caster<S, T>` from a concrete type `S` to a trait `T` implemented by it.
fn caster<T: ?Sized + 'static>(type_id: TypeId) -> Option<&'static Caster<T>>
{
    CASTER_MAP
        .get(&(type_id, TypeId::of::<Caster<T>>()))
        .and_then(|caster| caster.downcast_ref::<Caster<T>>())
}

/// `CastFrom` must be extended by a trait that wants to allow for casting into another trait.
///
/// It is used for obtaining a trait object for [`Any`] from a trait object for its sub-trait,
/// and blanket implemented for all `Sized + Any + 'static` types.
///
/// # Examples
/// ```ignore
/// trait Source: CastFrom {
///     ...
/// }
/// ```
pub trait CastFrom: Any + 'static
{
    /// Returns a `Box` of `Any`, which is backed by the type implementing this trait.
    fn box_any(self: Box<Self>) -> Box<dyn Any>;

    /// Returns an `Rc` of `Any`, which is backed by the type implementing this trait.
    fn rc_any(self: Rc<Self>) -> Rc<dyn Any>;
}

pub trait CastFromSync: CastFrom + Sync + Send + 'static
{
    fn arc_any(self: Arc<Self>) -> Arc<dyn Any + Sync + Send + 'static>;
}

impl<T: Sized + Any + 'static> CastFrom for T
{
    fn box_any(self: Box<Self>) -> Box<dyn Any>
    {
        self
    }

    fn rc_any(self: Rc<Self>) -> Rc<dyn Any>
    {
        self
    }
}

impl CastFrom for dyn Any + 'static
{
    fn box_any(self: Box<Self>) -> Box<dyn Any>
    {
        self
    }

    fn rc_any(self: Rc<Self>) -> Rc<dyn Any>
    {
        self
    }
}

impl CastFrom for dyn Any + Sync + Send + 'static
{
    fn box_any(self: Box<Self>) -> Box<dyn Any>
    {
        self
    }

    fn rc_any(self: Rc<Self>) -> Rc<dyn Any>
    {
        self
    }
}