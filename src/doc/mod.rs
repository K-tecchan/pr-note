use tera::{Context, Result, Tera};

#[derive(Debug)]
pub struct Doc {
    tera: Tera,
}

impl Doc {
    pub fn new() -> Self {
        Doc {
            tera: Tera::default(),
        }
    }

    pub fn render(
        &mut self,
        template_path: &str,
        prs: &Vec<crate::github::PullRequest>,
    ) -> Result<String> {
        self.tera
            .add_template_file(template_path, Some("template"))
            .unwrap();

        let mut context = Context::new();
        context.insert("prs", prs);
        self.tera.render("template", &context)
    }
}
