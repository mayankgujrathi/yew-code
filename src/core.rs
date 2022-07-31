pub use crate::minimal::{comp, comp_with_key, ui};

use std::{borrow::Cow, rc::Rc};
use yew::{
  virtual_dom::{AttrValue, Key, Listener, ListenerKind, VComp, VNode, VTag, VText},
  Component,
};

pub struct Ui {
  node: VNode,
}

impl Ui {
  #[inline]
  pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
    Self {
      node: VNode::VTag(Box::new(VTag::new(name))),
    }
  }
  #[inline]
  pub fn from(node: VNode) -> Self {
    Self { node }
  }
  #[inline]
  pub fn comp<COMP>(props: COMP::Properties) -> Self
  where
    COMP: Component,
  {
    Self {
      node: VNode::VComp(VComp::new::<COMP>(
        std::rc::Rc::new(props),
        yew::html::NodeRef::default(),
        None,
      )),
    }
  }
  #[inline]
  pub fn comp_with_key<COMP>(props: COMP::Properties, key: Key) -> Self
  where
    COMP: Component,
  {
    Self {
      node: VNode::VComp(VComp::new::<COMP>(
        std::rc::Rc::new(props),
        yew::html::NodeRef::default(),
        Some(key),
      )),
    }
  }
  #[inline]
  pub fn child(mut self, tag: Self) -> Self {
    match &mut self.node {
      VNode::VTag(root) => root.add_child(tag.node),
      _ => unimplemented!(),
    };
    self
  }
  #[inline]
  pub fn childern(mut self, children: impl IntoIterator<Item = Self>) -> Self {
    match &mut self.node {
      VNode::VTag(root) => root.add_children(children.into_iter().map(|x| x.node.clone())),
      _ => unimplemented!(),
    };
    self
  }
  #[inline]
  pub fn text(mut self, text: &'static str) -> Self {
    match &mut self.node {
      VNode::VTag(root) => root.add_child(VNode::VText(VText::new(text))),
      _ => unimplemented!(),
    };
    self
  }
  #[inline]
  pub fn attr(mut self, key: &'static str, value: impl Into<AttrValue>) -> Self {
    match &mut self.node {
      VNode::VTag(root) => root.add_attribute(key, value),
      _ => unimplemented!(),
    };
    self
  }
  #[inline]
  pub fn icss(self, value: impl Into<AttrValue>) -> Self {
    self.attr("style", value)
  }
  #[inline]
  pub fn vnode(self) -> VNode {
    self.node
  }
  #[inline]
  pub fn add_events(mut self, listeners: Box<[Option<Rc<dyn Listener>>]>) -> Self {
    match &mut self.node {
      VNode::VTag(root) => root.set_listener(listeners),
      _ => unimplemented!(),
    };
    self
  }
}

type Handler = Box<dyn Fn(web_sys::Event)>;

pub struct UiListener {
  pvt_kind: ListenerKind,
  pvt_handle: Handler,
  pvt_passive: bool,
}

impl UiListener {
  pub fn new(kind: ListenerKind, handle: Handler, passive: bool) -> Self {
    Self {
      pvt_kind: kind,
      pvt_handle: handle,
      pvt_passive: passive,
    }
  }
  pub fn from(kind: ListenerKind, handle: Handler) -> Option<Rc<dyn Listener>> {
    Some(Rc::new(UiListener::new(kind, handle, false)))
  }
}

impl Listener for UiListener {
  fn kind(&self) -> ListenerKind {
    self.pvt_kind
  }
  fn handle(&self, event: web_sys::Event) {
    (self.pvt_handle)(event);
  }
  fn passive(&self) -> bool {
    self.pvt_passive
  }
}

pub fn listener(kind: ListenerKind, handle: Handler) -> Option<Rc<dyn Listener>> {
  UiListener::from(kind, handle)
}

pub fn listener_passive(kind: ListenerKind, handle: Handler) -> Option<Rc<dyn Listener>> {
  Some(Rc::new(UiListener::new(kind, handle, true)))
}
