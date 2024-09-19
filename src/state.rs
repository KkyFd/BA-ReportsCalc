pub trait State<Element = Self> {
    fn load_from_file(&self) -> Result<Element, Box<dyn std::error::Error>>;
    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}
