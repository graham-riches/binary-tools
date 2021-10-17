#[cfg(test)]
mod tests;

pub trait HtmlElement {
    fn start_tag() -> String;
    fn end_tag() -> String;
    fn get_content(&self) -> String;

    fn to_html(&self) -> String {
        let mut s = Self::start_tag();
        s += &self.get_content();
        s += &Self::end_tag();
        s
    }
}

pub struct H1 {
    heading: String
}

impl H1 {
    fn new(heading: &str) -> Self {
        H1 { heading: heading.to_string() }
    }
}

impl HtmlElement for H1 {
    fn start_tag() -> String {
        "<h1>".to_string()
    }

    fn end_tag() -> String {
        "</h1>".to_string()
    }

    fn get_content(&self) -> String {
        self.heading.clone()
    }
}