use crate::impls::log::Log;
use teloc::{
    container::{Init, InstanceContainer},
    reexport::frunk::HCons,
    Resolver, ServiceProvider,
};

type ServiceContainer<Parent, T, Deps> = ServiceProvider<Parent, HCons<InstanceContainer<T>, Deps>>;

#[derive(Debug)]
pub struct PaDi<Parent, T, Deps> {
    service_container: ServiceContainer<Parent, T, Deps>,
}

impl<Parent, T, Deps> PaDi<Parent, T, Deps>
where
    InstanceContainer<T>: Init<Data = T>,
{
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(service_container: ServiceContainer<Parent, T, Deps>) -> Self {
        let log: Log = service_container.resolve();
        Self { service_container }
    }
}
