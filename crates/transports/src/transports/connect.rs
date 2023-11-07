use crate::{BoxTransport, Pbf, RpcClient, Transport, TransportError};

/// Connection details for a transport.
///
/// This object captures the information necessary to establish a transport,
/// and may encapsulate reconnection logic.
///
/// ## Why implement `TransportConnect`?
///
/// Users may want to implement transport-connect for the following reasons:
/// - You want to customize a [`reqwest::Client`] before using it.
/// - You need to provide special authentication information to a remote
///   provider.
/// - You have implemented a custom [`Transport`].
/// - You require a specific websocket reconnection strategy.
pub trait TransportConnect: Sized + Send + Sync + 'static {
    /// The transport type that is returned by `connect`.
    type Transport: Transport + Clone;

    /// Returns `true`` if the transport connects to a local resource.
    fn is_local(&self) -> bool;

    /// Connect to the transport, returning a `Transport` instance.
    fn get_transport<'a: 'b, 'b>(&self) -> Pbf<'b, Self::Transport, TransportError>;

    /// Connect to the transport, wrapping it into a `RpcClient` instance.
    fn connect<'a: 'b, 'b>(&'a self) -> Pbf<'b, RpcClient<Self::Transport>, TransportError> {
        Box::pin(async move {
            self.get_transport()
                .await
                .map(|t| RpcClient::new(t, self.is_local()))
        })
    }
}

/// Connection details for a transport that can be boxed.
///
/// This trait is implemented for [`TransportConnect`] implementors that
/// produce a boxable transport. It can be used to create a boxed transport
/// without knowing the exact type of the transport.
///
/// This trait is separate from `TransportConnect`` to hide the associated type
/// in when this trait is a trai object. It is intended to allow creation of
/// several unlike transports or clients at once. E.g.
/// `Vec<&dyn BoxTransportConnect>.into_iter().map(|t| t.connect_boxed())`.
pub trait BoxTransportConnect {
    /// Returns `true`` if the transport is a local transport.
    fn is_local(&self) -> bool;

    /// Connect to a transport, and box it.
    fn get_boxed_transport<'a: 'b, 'b>(&'a self) -> Pbf<'b, BoxTransport, TransportError>;

    /// Connect to a transport, and box it, wrapping it into a `RpcClient`.
    fn connect_boxed<'a: 'b, 'b>(&'a self) -> Pbf<'b, RpcClient<BoxTransport>, TransportError>;
}

impl<T> BoxTransportConnect for T
where
    T: TransportConnect,
{
    fn is_local(&self) -> bool {
        TransportConnect::is_local(self)
    }

    fn get_boxed_transport<'a: 'b, 'b>(&'a self) -> Pbf<'b, BoxTransport, TransportError> {
        Box::pin(async move { self.get_transport().await.map(Transport::boxed) })
    }

    fn connect_boxed<'a: 'b, 'b>(&'a self) -> Pbf<'b, RpcClient<BoxTransport>, TransportError> {
        Box::pin(async move {
            self.get_boxed_transport()
                .await
                .map(|boxed| RpcClient::new(boxed, self.is_local()))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn __compile_check(_: Box<dyn BoxTransportConnect>) {
        todo!()
    }
}
