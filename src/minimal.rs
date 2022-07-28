use crate::core::Ui;
use std::borrow::Cow;
use yew::{
  virtual_dom::{AttrValue, Key},
  Component,
};

#[inline]
pub fn ui(name: impl Into<Cow<'static, str>>) -> Ui {
  Ui::new(name)
}

#[inline]
pub fn view() -> Ui {
  Ui::new("div")
}

#[inline]
pub fn cell() -> Ui {
  Ui::new("span")
}

#[inline]
pub fn comp<COMP>(props: COMP::Properties) -> Ui
where
  COMP: Component,
{
  Ui::comp::<COMP>(props)
}

#[inline]
pub fn comp_with_key<COMP>(props: COMP::Properties, key: Key) -> Ui
where
  COMP: Component,
{
  Ui::comp_with_key::<COMP>(props, key)
}

#[inline]
pub fn img(src: impl Into<AttrValue>) -> Ui {
  Ui::new("img").attr("src", src)
}

#[inline]
pub fn video() -> Ui {
  Ui::new("video")
}

#[inline]
pub fn audio() -> Ui {
  Ui::new("audio")
}

#[inline]
pub fn iframe(src: impl Into<AttrValue>) -> Ui {
  Ui::new("iframe").attr("src", src)
}

#[inline]
pub fn a(href: impl Into<AttrValue>) -> Ui {
  Ui::new("a").attr("href", href)
}

#[inline]
pub fn object(data: impl Into<AttrValue>) -> Ui {
  Ui::new("object").attr("data", data)
}

#[inline]
pub fn button(text: &'static str) -> Ui {
  Ui::new("button").text(text)
}
