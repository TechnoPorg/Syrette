//! Scope configurator for a binding for types inside of a [`IAsyncDIContainer`].
//!
//! [`IAsyncDIContainer`]: crate::di_container::asynchronous::IAsyncDIContainer
use std::marker::PhantomData;
use std::sync::Arc;

use crate::di_container::asynchronous::binding::when_configurator::AsyncBindingWhenConfigurator;
use crate::di_container::asynchronous::IAsyncDIContainer;
use crate::errors::async_di_container::AsyncBindingScopeConfiguratorError;
use crate::interfaces::async_injectable::AsyncInjectable;
use crate::provider::r#async::{AsyncSingletonProvider, AsyncTransientTypeProvider};
use crate::ptr::ThreadsafeSingletonPtr;

/// Scope configurator for a binding for type 'Interface' inside a [`IAsyncDIContainer`].
///
/// [`IAsyncDIContainer`]: crate::di_container::asynchronous::IAsyncDIContainer
pub struct AsyncBindingScopeConfigurator<Interface, Implementation, DIContainerType>
where
    Interface: 'static + ?Sized + Send + Sync,
    Implementation: AsyncInjectable<DIContainerType>,
    DIContainerType: IAsyncDIContainer,
{
    di_container: Arc<DIContainerType>,
    interface_phantom: PhantomData<Interface>,
    implementation_phantom: PhantomData<Implementation>,
}

impl<Interface, Implementation, DIContainerType>
    AsyncBindingScopeConfigurator<Interface, Implementation, DIContainerType>
where
    Interface: 'static + ?Sized + Send + Sync,
    Implementation: AsyncInjectable<DIContainerType>,
    DIContainerType: IAsyncDIContainer,
{
    pub(crate) fn new(di_container: Arc<DIContainerType>) -> Self
    {
        Self {
            di_container,
            interface_phantom: PhantomData,
            implementation_phantom: PhantomData,
        }
    }

    /// Configures the binding to be in a transient scope.
    ///
    /// This is the default.
    pub async fn in_transient_scope(
        &self,
    ) -> AsyncBindingWhenConfigurator<Interface, DIContainerType>
    {
        self.di_container
            .set_binding::<Interface>(
                None,
                Box::new(
                    AsyncTransientTypeProvider::<Implementation, DIContainerType>::new(),
                ),
            )
            .await;

        AsyncBindingWhenConfigurator::new(self.di_container.clone())
    }

    /// Configures the binding to be in a singleton scope.
    ///
    /// # Errors
    /// Will return Err if resolving the implementation fails.
    pub async fn in_singleton_scope(
        &self,
    ) -> Result<
        AsyncBindingWhenConfigurator<Interface, DIContainerType>,
        AsyncBindingScopeConfiguratorError,
    >
    {
        let singleton: ThreadsafeSingletonPtr<Implementation> =
            ThreadsafeSingletonPtr::from(
                Implementation::resolve(&self.di_container, Vec::new())
                    .await
                    .map_err(
                        AsyncBindingScopeConfiguratorError::SingletonResolveFailed,
                    )?,
            );

        self.di_container
            .set_binding::<Interface>(
                None,
                Box::new(AsyncSingletonProvider::new(singleton)),
            )
            .await;

        Ok(AsyncBindingWhenConfigurator::new(self.di_container.clone()))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::test_utils::{mocks, subjects_async};

    #[tokio::test]
    async fn in_transient_scope_works()
    {
        let mut di_container_mock =
            mocks::async_di_container::MockAsyncDIContainer::new();

        di_container_mock
            .expect_set_binding::<dyn subjects_async::IUserManager>()
            .withf(|name, _provider| name.is_none())
            .return_once(|_name, _provider| ())
            .once();

        let binding_scope_configurator = AsyncBindingScopeConfigurator::<
            dyn subjects_async::IUserManager,
            subjects_async::UserManager,
            mocks::async_di_container::MockAsyncDIContainer,
        >::new(Arc::new(di_container_mock));

        binding_scope_configurator.in_transient_scope().await;
    }

    #[tokio::test]
    async fn in_singleton_scope_works()
    {
        let mut di_container_mock =
            mocks::async_di_container::MockAsyncDIContainer::new();

        di_container_mock
            .expect_set_binding::<dyn subjects_async::IUserManager>()
            .withf(|name, _provider| name.is_none())
            .return_once(|_name, _provider| ())
            .once();

        let binding_scope_configurator = AsyncBindingScopeConfigurator::<
            dyn subjects_async::IUserManager,
            subjects_async::UserManager,
            mocks::async_di_container::MockAsyncDIContainer,
        >::new(Arc::new(di_container_mock));

        assert!(matches!(
            binding_scope_configurator.in_singleton_scope().await,
            Ok(_)
        ));
    }
}
