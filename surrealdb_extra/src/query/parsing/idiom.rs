use surrealdb::sql::{Idiom, Part};

#[derive(Debug, Clone)]
pub struct ExtraIdiom(pub Idiom);

impl From<Idiom> for ExtraIdiom {
    fn from(value: Idiom) -> Self {
        Self(value)
    }
}

impl From<&str> for ExtraIdiom {
    fn from(value: &str) -> Self {
        let part: Vec<Part> = value
            .split('.')
            .map(Part::from)
            .collect();


        let mut idiom = Idiom::default();
        idiom.0 = part;

        Self(idiom)
    }
}


impl From<String> for ExtraIdiom {
    fn from(value: String) -> Self {
        let part: Vec<Part> = value
            .split('.')
            .map(Part::from)
            .collect();

        let mut idiom = Idiom::default();
        idiom.0 = part;

        Self(idiom)
    }
}
