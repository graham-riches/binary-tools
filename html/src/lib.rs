use html_element_derive::HtmlElement;

#[cfg(test)]
mod tests;

/// Trait required for any HtmlElement that provides
/// an interface to return the content of the HTML element
pub trait HtmlContent {
    fn get_content(&self) -> String;
}


pub trait HtmlElement: HtmlContent {
    fn start_tag() -> String;
    fn end_tag() -> String;

    fn to_html(&self) -> String {
        let mut s = Self::start_tag();
        s += &self.get_content();
        s += &Self::end_tag();
        s
    }
}

#[derive(HtmlElement)]
pub struct H1 {
    heading: String
}

impl HtmlContent for H1 {
    fn get_content(&self) -> String {
        self.heading.clone()
    }
}

impl H1 {
    pub fn new(heading: &str) -> Self {
        H1{ heading: heading.to_string() }
    }
}