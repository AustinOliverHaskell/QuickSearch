pub struct FileSearch;
pub struct FileIndex {

}

impl FileSearch {
	pub fn create_index(root_path: &str) -> FileIndex {

		FileIndex {

		}
	}

	pub fn search(root_path: &str, search_string: &str) -> FileIndex {
		println!("Searching Root Path {:} for {:}", root_path, search_string);
		Self::create_index(root_path).search(root_path)
	}
}

impl FileIndex {
	pub fn search(self: &Self, root_path: &str) -> Self {
		FileIndex {

		}
	}
}