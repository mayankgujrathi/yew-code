pub use crate::minimal::{a, audio, button, cell as span, iframe, img, object, video, view as div};

pub mod table {
  use crate::core::Ui;
  #[inline]
  pub fn caption(text: &'static str) -> Ui {
    Ui::new("caption").text(text)
  }
  #[inline]
  pub fn table() -> Ui {
    Ui::new("table")
  }
  #[inline]
  pub fn tbody() -> Ui {
    Ui::new("tbody")
  }
  #[inline]
  pub fn thead() -> Ui {
    Ui::new("thead")
  }
  #[inline]
  pub fn tfoot() -> Ui {
    Ui::new("tfoot")
  }
  #[inline]
  pub fn tr() -> Ui {
    Ui::new("tr")
  }
  #[inline]
  pub fn td() -> Ui {
    Ui::new("td")
  }
  #[inline]
  pub fn th() -> Ui {
    Ui::new("th")
  }
}

pub mod form {
  use crate::core::Ui;
  use yew::virtual_dom::AttrValue;
  pub enum Method {
    Get,
    Post,
  }
  pub enum Type {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
  }
  #[inline]
  pub fn form(method: Method) -> Ui {
    use Method::*;
    Ui::new("form").attr(
      "method",
      match method {
        Get => "get",
        Post => "post",
      },
    )
  }
  #[inline]
  pub fn textarea() -> Ui {
    Ui::new("textarea")
  }
  #[inline]
  pub fn legend() -> Ui {
    Ui::new("legend")
  }
  #[inline]
  pub fn label() -> Ui {
    Ui::new("label")
  }
  #[inline]
  pub fn fieldset() -> Ui {
    Ui::new("fieldset")
  }
  #[inline]
  pub fn select() -> Ui {
    Ui::new("select")
  }
  #[inline]
  pub fn option(value: impl Into<AttrValue>) -> Ui {
    Ui::new("option").attr("value", value)
  }
  #[inline]
  pub fn input(ty: Type) -> Ui {
    use Type::*;
    Ui::new("input").attr(
      "type",
      match ty {
        Button => "button",
        Checkbox => "checkbox",
        Color => "color",
        Date => "date",
        DatetimeLocal => "datetime-local",
        Email => "email",
        File => "file",
        Hidden => "hidden",
        Image => "image",
        Month => "month",
        Number => "number",
        Password => "password",
        Radio => "radio",
        Range => "range",
        Reset => "reset",
        Search => "search",
        Submit => "submit",
        Tel => "tel",
        Text => "text",
        Time => "time",
        Url => "url",
        Week => "week",
      },
    )
  }
}

use crate::core::Ui;

#[inline]
pub fn header() -> Ui {
  Ui::new("header")
}

#[inline]
pub fn footer() -> Ui {
  Ui::new("footer")
}

#[inline]
pub fn mainsec() -> Ui {
  Ui::new("main")
}

#[inline]
pub fn section() -> Ui {
  Ui::new("section")
}

#[inline]
pub fn svg() -> Ui {
  Ui::new("svg")
}

#[inline]
pub fn ul() -> Ui {
  Ui::new("ul")
}

#[inline]
pub fn ol() -> Ui {
  Ui::new("ol")
}

#[inline]
pub fn li() -> Ui {
  Ui::new("li")
}

#[inline]
pub fn pre(text: &'static str) -> Ui {
  Ui::new("pre").text(text)
}

#[inline]
pub fn code(text: &'static str) -> Ui {
  Ui::new("code").text(text)
}

#[inline]
pub fn hr() -> Ui {
  Ui::new("hr")
}

#[inline]
pub fn br() -> Ui {
  Ui::new("br")
}

#[inline]
pub fn blockquote(text: &'static str) -> Ui {
  Ui::new("blockquote").text(text)
}

#[inline]
pub fn b(text: &'static str) -> Ui {
  Ui::new("b").text(text)
}

#[inline]
pub fn p(text: &'static str) -> Ui {
  Ui::new("p").text(text)
}

#[inline]
pub fn q(text: &'static str) -> Ui {
  Ui::new("q").text(text)
}

#[inline]
pub fn i(text: &'static str) -> Ui {
  Ui::new("i").text(text)
}

#[inline]
pub fn sub(text: &'static str) -> Ui {
  Ui::new("sub").text(text)
}

#[inline]
pub fn sup(text: &'static str) -> Ui {
  Ui::new("sup").text(text)
}

#[inline]
pub fn h1(text: &'static str) -> Ui {
  Ui::new("h1").text(text)
}

#[inline]
pub fn h2(text: &'static str) -> Ui {
  Ui::new("h2").text(text)
}

#[inline]
pub fn h3(text: &'static str) -> Ui {
  Ui::new("h3").text(text)
}

#[inline]
pub fn h4(text: &'static str) -> Ui {
  Ui::new("h4").text(text)
}

#[inline]
pub fn h5(text: &'static str) -> Ui {
  Ui::new("h5").text(text)
}

#[inline]
pub fn h6(text: &'static str) -> Ui {
  Ui::new("h6").text(text)
}

#[inline]
pub fn canvas() -> Ui {
  Ui::new("canvas")
}
