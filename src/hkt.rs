pub trait Hkt<A> { type App; }

pub type App<F, A> = <F as Hkt<A>>::App;
