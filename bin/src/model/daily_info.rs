use super::example::Example;

#[derive(Debug)]
pub struct QuestionBaseInfo {
    pub id: usize,
    pub link: String,
    pub name: String,
    pub slug: String,
}

impl QuestionBaseInfo {
    pub fn new(id: usize, link: String, name: String, slug: String) -> Self {
        QuestionBaseInfo {
            id,
            link,
            name,
            slug,
        }
    }
}

#[derive(Debug)]
pub struct QuestionInfo {
    pub base_info: QuestionBaseInfo,
    pub content: String,
    pub template: String,
    pub examples: Vec<Example>,
}

impl QuestionInfo {
    pub fn new(
        base_info: QuestionBaseInfo,
        content: String,
        template: String,
        examples: Vec<Example>,
    ) -> Self {
        QuestionInfo {
            base_info,
            content,
            template,
            examples,
        }
    }

    pub fn to_string(self) -> String {
        let line = self.content.trim().split('\n');

        let comment = line
            .into_iter()
            .map(|s| "/// ".to_string() + s)
            .collect::<Vec<_>>()
            .join("\n")
            .to_string();
        format!(
            "/// {}.{}\n///\n{}\n/// <a href=\"{}\">{}</a>\npub struct Solution;\n\n{}\n\n#[cfg(test)]\nmod test {{\n    use super::*;\n    #[test]\n    fn test_1() {{\n\n    }}\n}}",
            self.base_info.id, self.base_info.name, comment, self.base_info.link, self.base_info.name, self.template
        )
    }
}
