use syrette::DIContainer;

// Concrete implementations
use crate::animals::cat::Cat;
use crate::animals::cow::Cow;
use crate::animals::dog::Dog;
use crate::animals::human::Human;
//
// Interfaces
use crate::interfaces::cat::ICat;
use crate::interfaces::cow::{CowFactory, ICow};
use crate::interfaces::dog::IDog;
use crate::interfaces::human::IHuman;

pub fn bootstrap() -> DIContainer
{
    let mut di_container: DIContainer = DIContainer::new();

    di_container.bind::<dyn IDog>().to::<Dog>();
    di_container.bind::<dyn ICat>().to::<Cat>();
    di_container.bind::<dyn IHuman>().to::<Human>();

    di_container.bind::<CowFactory>().to_factory(&|moo_cnt| {
        let cow: Box<dyn ICow> = Box::new(Cow::new(moo_cnt));
        cow
    });

    di_container
}