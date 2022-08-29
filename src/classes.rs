#![allow(dead_code)]

use crate::hkt::*;

pub trait Functor {
  fn map<A, B>(f: impl Fn(A) -> B, fa: App<Self, A>) -> App<Self, B> where Self: Hkt<A> + Hkt<B>;
}

pub trait Applicative: Functor {
  fn pure<A>(a: A) -> App<Self, A> where Self: Hkt<A>;

  fn map2<A, B, C>(f: impl Fn(&A, &B) -> C, fa: App<Self, A>, fb: App<Self, B>) -> App<Self, C>
  where Self: Hkt<A> + Hkt<B> + Hkt<C>;
}

pub trait Monad: Applicative {
  fn bind<A, B>(f: impl Fn(&A) -> App<Self, B>, m: App<Self, A>) -> App<Self, B> where Self: Hkt<A> + Hkt<B>;
}

pub fn when<M: Monad + Hkt<A> + Hkt<()>, A>(b: bool, m: App<M, A>) -> App<M, ()> {
  if b {
    M::map(|_: A| (), m)
  } else {
    M::pure(())
  }
}

pub trait Foldable {
  fn foldl<A, B>(f: impl Fn(B, A) -> B, z: B, t: App<Self, A>) -> B where Self: Hkt<A>;
  fn foldr<A, B>(f: impl Fn(A, B) -> B, z: B, t: App<Self, A>) -> B where Self: Hkt<A>;
}

pub trait Traversable: Functor + Foldable {
  fn traverse<F, A, B>(f: impl Fn(A) -> App<F, B>, t: App<Self, A>) -> App<F, App<Self, B>>
  where
    Self: Hkt<A> + Hkt<B>,
    F: Applicative + Hkt<B> + Hkt<App<Self, B>>;
}
