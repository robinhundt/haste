use std::fmt::{Display, Write};

#[derive(Debug)]
pub struct Label {
    parts: Vec<String>,
}

impl Label {
    pub fn new(root: &str) -> Self {
        Self {
            parts: vec![root.to_string()],
        }
    }

    pub fn with_part(mut self, part: impl Display) -> Self {
        self.parts.push(part.to_string());
        self
    }
}

impl<'a> From<&'a str> for Label {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((first, rest)) = self.parts.split_first() {
            f.write_str(first)?;

            for part in rest {
                f.write_char('/')?;
                f.write_str(part)?;
            }
        }

        Ok(())
    }
}
