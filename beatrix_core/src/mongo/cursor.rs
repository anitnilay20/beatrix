use futures::Stream;
use mongodb::Cursor;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use super::{
    error::{Error, Result},
    MongoModel,
};

pub struct ModelCursor<T> {
    cursor: Cursor<T>,
    model: PhantomData<T>,
}

impl<T: MongoModel> ModelCursor<T> {
    pub(crate) fn new(cursor: Cursor<T>) -> Self {
        Self {
            model: std::marker::PhantomData,
            cursor,
        }
    }
}

impl<T> Unpin for ModelCursor<T> {}

impl<T: MongoModel> Stream for ModelCursor<T> {
    type Item = Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let doc = match Pin::new(&mut self.cursor).poll_next(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(Err(err))) => return Poll::Ready(Some(Err(Error::from(err)))),
            Poll::Ready(Some(Ok(doc))) => doc,
        };

        match MongoModel::from_bson(doc.to_document()?) {
            Ok(model) => Poll::Ready(Some(Ok(model))),
            Err(err) => Poll::Ready(Some(Err(err)))
        }
    }
}
