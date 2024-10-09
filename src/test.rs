use strum::*;
use leptos::prelude::*;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::{FromStr, ParseBoolError};
use leptos_router::hooks::query_signal;
use crate::test::Nation::*;
use leptos_router::params::Params;
use itertools::Itertools;
#[derive(Clone, Debug, PartialEq, Default, Params)]
pub struct StationUrlFilter {
    pub river: Option<StringParams>,
    pub nation: Option<NationParams>,
    pub activity: Option<ActivityParam>,
}

#[component]
pub fn StationInputForm() -> impl IntoView {
    //https://github.com/leptos-rs/leptos/issues/2369
    let nation: RwSignal<Vec<Nation>> = RwSignal::new(Default::default());
    let river: RwSignal<Vec<String>> = RwSignal::new(Default::default());
    let activity: RwSignal<bool> = RwSignal::new(Default::default());

    let (read_r, set_r) = query_signal::<StringParams>("rivers");
    let (read_n, set_n) = query_signal::<NationParams>("nation");
    let (read_a, set_a) = query_signal::<ActivityParam>("activity");
    let submit_fn = move |_| {
        let r = vec!["a".to_string(),  "b".to_string(), "c".to_string()];
        let n = vec![A, B, C, D];
        let a = false;
        set_r.set(Some(StringParams(r)));
        set_a.set(Some(ActivityParam(a)));
        set_n.set(Some(NationParams(n)));

    };

    view! {
        <button on:click=submit_fn>"Click Me: " </button>
    }
}


pub fn parse_vec_from_str<T: FromStr>(s: &str) ->Result<Vec<T>, T::Err>{
    let mut items = vec![];
    for p in s.split(s){
        let item = T::from_str(p)?;
        items.push(item)
    }
    Ok(items)
}

#[derive(Display, EnumString, Clone, Debug, Eq, PartialEq)]
pub enum Nation{
    A,B,C,D,E
}

#[derive( Clone, Debug, PartialEq, Default)]
pub struct NationParams(pub Vec<Nation>);
#[derive( Clone, Debug, PartialEq, Default)]
pub struct StringParams(pub Vec<String>);

#[derive( Clone, Debug, PartialEq, Default)]
pub struct ActivityParam(pub bool);



impl FromStr for NationParams{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_vec_from_str(s)?))
    }
}

impl FromStr for StringParams{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(",").map(ToString::to_string).collect_vec()))
    }
}

impl FromStr for ActivityParam{
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ActivityParam(bool::from_str(s)?))
    }
}


impl Display for NationParams{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(ToString::to_string).join(","))
    }
}
impl Display for StringParams{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}
impl Display for ActivityParam{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
pub trait EmptyToNone: Sized{
    fn empty_to_none(self)->Option<Self>;
}
impl <T> EmptyToNone for [T] where [T]: Sized {
    fn empty_to_none(self) -> Option<Self> {
        match self.is_empty(){
            true => None::<Self>,
            false => Some(self)
        }
    }
}
impl <T> EmptyToNone for Vec<T> where T: Sized {
    fn empty_to_none(self) -> Option<Self<>> {
        match self.is_empty(){
            true => None::<Self>,
            false => Some(self)
        }
    }
}

