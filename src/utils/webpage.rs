use maud::{Markup, html};

#[derive(Debug)]
pub struct WebPageBuilder {
    pub title: String,
    pub subtitle: Option<String>,
    pub body: Markup,
}

impl WebPageBuilder {
    pub fn new() -> Self {
        Self {
            title: "Walrus".into(),
            subtitle: None,
            body: html! {},
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: Option<impl Into<String>>) -> Self {
        self.subtitle = match subtitle {
            Some(s) => Some(s.into()),
            _ => None,
        };
        self
    }

    pub fn body(mut self, body: impl Into<Markup>) -> Self {
        self.body = body.into();
        self
    }

    pub fn build(self) -> Markup {
        let gen_title = match self.subtitle {
            Some(sub) => format!("{} | {}", sub, self.title),
            _ => self.title,
        };

        html! {
            head {
                meta charset="utf-8";
                title { (gen_title) }
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js" integrity="sha384-ZBXiYtYQ6hJ2Y0ZNoYuI+Nq5MqWBr+chMrS/RkXpNzQCApHEhOt2aY8EJgqwHLkJ" crossorigin="anonymous" {}
                link rel="stylesheet" href="/public/output.css";
            }
            body."flex flex-col items-center"{
               (self.body)
            }
        }
    }
}
