use crate::interfaces::factory::IFactory;
use crate::libs::intertrait::CastFrom;
use crate::ptr::InterfacePtr;

pub trait AnyFactory: CastFrom {}

pub struct CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
    func: &'static dyn Fn<Args, Output = InterfacePtr<ReturnInterface>>,
}

impl<Args, ReturnInterface> CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
    pub fn new(
        func: &'static dyn Fn<Args, Output = InterfacePtr<ReturnInterface>>,
    ) -> Self
    {
        Self { func }
    }
}

impl<Args, ReturnInterface> IFactory<Args, ReturnInterface>
    for CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
}

impl<Args, ReturnInterface> Fn<Args> for CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output
    {
        self.func.call(args)
    }
}

impl<Args, ReturnInterface> FnMut<Args> for CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output
    {
        self.call(args)
    }
}

impl<Args, ReturnInterface> FnOnce<Args> for CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
    type Output = InterfacePtr<ReturnInterface>;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output
    {
        self.call(args)
    }
}

impl<Args, ReturnInterface> AnyFactory for CastableFactory<Args, ReturnInterface>
where
    Args: 'static,
    ReturnInterface: 'static + ?Sized,
{
}
