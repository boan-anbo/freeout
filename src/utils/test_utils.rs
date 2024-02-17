#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;

    pub fn get_test_folder() -> PathBuf {
        let tests_folder = std::env::current_dir().unwrap().join("tests");
        tests_folder
    }

    pub fn get_fixture_folder() -> PathBuf {
        let fixture_folder = get_test_folder().join("fixtures");
        fixture_folder
    }

    pub fn get_temp_folder() -> PathBuf {
        let temp_folder = get_test_folder().join("temp");
        // create temp folder if it does not exist
        if !temp_folder.exists() {
            std::fs::create_dir_all(&temp_folder).unwrap();
        }
        temp_folder
    }

    pub fn get_markdown_folder() -> PathBuf {
        let markdown_folder = get_fixture_folder().join("markdown");
        markdown_folder
    }


    pub fn md_get_simple_eng() -> PathBuf {
        let markdown_file = get_markdown_folder().join("simple_eng.md");
        markdown_file
    }

    pub fn md_get_minimal_eng() -> PathBuf {
        let markdown_file = get_markdown_folder().join("minimal_eng.md");
        markdown_file
    }


    pub fn md_get_long_chinese() -> PathBuf {
        let markdown_file = get_markdown_folder().join("long_chinese.md");
        markdown_file
    }

}
