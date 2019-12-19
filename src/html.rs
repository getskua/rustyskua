use super::SavePage;

pub struct HTMLPage {
    body: String
}

impl SavePage for HTMLPage {
    fn compute_output(&self) -> String {}
}
