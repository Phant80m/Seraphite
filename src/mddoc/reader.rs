pub struct Pages {
    pub current_page: usize,
    pub pages: Vec<String>,
}
impl Pages {
    pub fn construct() -> Self {
        let mut pages = Vec::new();

        let mut page_number = 1;
        loop {
            let file_path = format!(
                "{}/dotfiles/docs/{}.md",
                std::env::var("HOME").unwrap(),
                page_number
            );
            match std::fs::read_to_string(&file_path) {
                Ok(content) => {
                    pages.push(content);
                    page_number += 1;
                }
                Err(_) => break,
            }
        }

        Self {
            current_page: 0,
            pages,
        }
    }

    pub fn current_page_content(&self) -> String {
        self.pages[self.current_page].clone()
    }
}
