use poprako_orchestra::Step;

use crate::prom::oper::Defer;

pub mod oper;

pub trait Prom<C, I, P>: for<'a> Step<Defer<'a, I, P>, C> {}

impl<T, C, I, P> Prom<C, I, P> for T where T: for<'a> Step<Defer<'a, I, P>, C> {}

