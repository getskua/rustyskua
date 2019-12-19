use tera::{Context, Tera};

use super::SavePage;

pub struct HTMLPage {
    body: String,
    template_location: String,
}

impl SavePage for HTMLPage {
    fn compute_output(&self) -> String {
        let mut tera = match Tera::new(&self.template_location) {
            Ok(t) => {}
            Err(e) => {
                panic!(e)
                ::std::process::exit(1)
            }
        };
        let mut context = Context::new();
        context.insert("body", &self.body);
        return tera.render(&self.template_location, &context);
    }
}
