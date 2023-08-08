use poem_openapi::Object;

#[derive(Object)]
#[oai(rename_all = "camelCase")]
pub struct Browse {
    page_number: u32,
    #[oai(default = "default_count")]
    count: u32,
}

impl Browse {
    pub fn page_number(&self) -> u32 {
        self.page_number
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

fn default_count() -> u32 {
    10
}
