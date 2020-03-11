use std::hash::Hash;
use serde::{Serialize, Deserialize};

pub trait Token: Eq + Hash + Serialize + Deserialize<'static> {
}

pub trait Locator: Eq + Hash + Serialize + Deserialize<'static> {
}

pub trait LimiterCondition {
    type Req;
    type Res;

    fn pre_cond(&mut self, req: &Self::Req) -> bool;

    fn post_cond(&mut self, res: &Self::Res) -> bool;
}

pub trait Limiter {
    type Locator: Locator;
    type ToTokens: ToTokens<Self::Locator>;
    type Condition: LimiterCondition;
}

pub trait ToTokens<Loc> {
    type Token: Token;

    fn to_tokens(&self, loc: Loc) -> Vec<Self::Token>;
}
