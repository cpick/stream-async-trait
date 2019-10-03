use async_trait::async_trait;
use futures::{
    future::{self, LocalBoxFuture},
    stream, StreamExt,
    executor::block_on_stream,
};

#[async_trait]
pub trait Make {
    // working
    fn make<'a, T: Default + 'a>(&self) -> LocalBoxFuture<'a, T> {
        Box::pin(future::ready(T::default()))
    }


    // broken

    // async fn make<'a, T: Default + 'a>(&self) -> T {
    //     T::default()
    // }

    // async fn make<T: Default>(&self) -> T {
    //     T::default()
    // }

    // async fn make<T: Default + 'async_trait>(&self) -> T {
    //     T::default()
    // }

    // async fn make<'a, T: Default + 'async_trait>(&'a self) -> T {
    //     T::default()
    // }

    // async fn make<'a, T: Default + 'a>(&'a self) -> T {
    //     T::default()
    // }

    // async fn make<'a, T: Default + 'a>(&'async_trait self) -> T {
    //     T::default()
    // }

    // async fn make<T: Default + 'async_trait>(&'async_trait self) -> T {
    //     T::default()
    // }

    // async fn make<T: Default>(&'async_trait self) -> T {
    //     T::default()
    // }
}

struct Foo {}
impl Make for Foo {}

fn main() {
    let f = Foo {};
    block_on_stream(stream::empty().map(move |_: ()| f.make::<bool>()));
}

