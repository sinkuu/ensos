use std::hash::Hash;
use serde::{Serialize, Deserialize};

pub trait Token: Eq + Hash + Serialize + Deserialize<'static> {
}

pub trait Locator: Eq + Hash {
}

pub trait Cond {
    type Req;
    type Res;
    type ResErr;
    type ReqCtx;

    fn pre(&mut self, req: &Self::Req) -> Option<Self::ReqCtx>;
    fn post(&mut self, res: Result<&Self::Res, &Self::ResErr>, ctx: Self::ReqCtx) -> bool;
}

pub struct Limiter<Locator, Token, Req, Res, ResErr, ReqCtx> {
    to_tokens: Box<dyn ToTokens<Locator, Token = Token>>,
    max: usize,
    cond: Box<dyn Cond<Req = Req, Res = Res, ResErr = ResErr, ReqCtx = ReqCtx>>,
}

pub trait ToTokens<Loc> {
    type Token: Token;

    fn to_tokens(&self, loc: Loc) -> Vec<Self::Token>;
}
