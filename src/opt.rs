use crate::hkt::*;
use crate::classes::*;

pub enum Opt { }

impl<A> Hkt<A> for Opt {
  type App = Option<A>;
}

impl Functor for Opt {
  fn map<A, B>(f: impl Fn(A) -> B, fa: App<Self, A>) -> App<Self, B> {
    fa.map(f)
  }
}

impl Applicative for Opt {
  fn pure<A>(a: A) -> App<Self, A> {
    Some(a)
  }

  fn map2<A, B, C>(f: impl Fn(&A, &B) -> C, fa: App<Self, A>, fb: App<Self, B>) -> App<Self, C> {
    fa.and_then(|a| fb.and_then(|b| Some(f(&a, &b)))) // Option::zip is still unstable
  }
}

impl Monad for Opt {
  fn bind<A, B>(f: impl Fn(&A) -> App<Self, B>, m: App<Self, A>) -> App<Self, B> {
    m.and_then(|a| f(&a))
  }
}

impl Foldable for Opt {
  fn foldl<A, B>(f: impl Fn(B, A) -> B, z: B, t: App<Self, A>) -> B {
    match t {
      None => z,
      Some(a) => f(z, a)
    }
  }

  fn foldr<A, B>(f: impl Fn(A, B) -> B, z: B, t: App<Self, A>) -> B {
    match t {
      None => z,
      Some(a) => f(a, z)
    }
  }
}

impl Traversable for Opt {
  fn traverse<F, A, B>(f: impl Fn(A) -> App<F, B>, t: App<Self, A>) -> App<F, App<Self, B>>
  where F: Applicative + Hkt<B> + Hkt<App<Self, B>> {
    match t {
      None => F::pure(None),
      Some(a) => F::map(Some, f(a))
    }
  }
}
