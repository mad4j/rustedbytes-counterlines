// language.rs - Language detection and comment syntax definitions
// Implements: REQ-3.1, REQ-3.2, REQ-3.3, REQ-3.4, REQ-4.2, REQ-4.3

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub extensions: Vec<String>,
    pub single_line_comment: Vec<String>,
    pub multi_line_comment: Vec<(String, String)>,
    pub nested_comments: bool, // REQ-4.3: Nested comments support
    pub preprocessor_prefix: Option<String>, // REQ-4.5: Preprocessor directives
}

#[derive(Debug, Clone)]
pub struct LanguageDetector {
    languages: HashMap<String, Language>,
    extension_map: HashMap<String, String>,
    overrides: HashMap<String, String>, // REQ-3.4: Language overrides
}

impl LanguageDetector {
    /// REQ-3.3: Load language definitions from configuration
    pub fn new() -> Self {
        let mut detector = Self {
            languages: HashMap::new(),
            extension_map: HashMap::new(),
            overrides: HashMap::new(),
        };
        detector.load_default_languages();
        detector
    }

    /// REQ-3.3: Load additional language definitions
    pub fn load_from_config(&mut self, config_path: &Path) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(config_path)?;
        let languages: HashMap<String, Language> = toml::from_str(&content)
            .map_err(|e| crate::error::SlocError::InvalidConfig(e.to_string()))?;

        for (key, lang) in languages {
            self.add_language(key, lang);
        }
        Ok(())
    }

    /// REQ-3.4: Add language override
    pub fn add_override(&mut self, extension: String, language: String) {
        self.overrides.insert(extension, language);
    }

    /// REQ-3.2: Detect language based on file extension
    pub fn detect(&self, path: &Path) -> Option<&Language> {
        let ext = path.extension()?.to_str()?;

        // Check overrides first (REQ-3.4)
        if let Some(lang_name) = self.overrides.get(ext) {
            return self.languages.get(lang_name);
        }

        // Then check extension map
        let lang_name = self.extension_map.get(ext)?;
        self.languages.get(lang_name)
    }

    fn add_language(&mut self, key: String, language: Language) {
        for ext in &language.extensions {
            self.extension_map.insert(ext.clone(), key.clone());
        }
        self.languages.insert(key, language);
    }

    /// REQ-3.1: Support multiple programming languages
    fn load_default_languages(&mut self) {
        // Rust
        self.add_language(
            "rust".to_string(),
            Language {
                name: "Rust".to_string(),
                extensions: vec!["rs".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: true, // REQ-4.3: Rust supports nested comments
                preprocessor_prefix: None,
            },
        );

        // C/C++
        self.add_language(
            "c".to_string(),
            Language {
                name: "C".to_string(),
                extensions: vec!["c".to_string(), "h".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: Some("#".to_string()), // REQ-4.5
            },
        );

        self.add_language(
            "cpp".to_string(),
            Language {
                name: "C++".to_string(),
                extensions: vec![
                    "cpp".to_string(),
                    "cc".to_string(),
                    "cxx".to_string(),
                    "hpp".to_string(),
                    "hh".to_string(),
                    "hxx".to_string(),
                ],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: Some("#".to_string()),
            },
        );

        // Python
        self.add_language(
            "python".to_string(),
            Language {
                name: "Python".to_string(),
                extensions: vec!["py".to_string(), "pyw".to_string()],
                single_line_comment: vec!["#".to_string()],
                multi_line_comment: vec![
                    ("'''".to_string(), "'''".to_string()),
                    ("\"\"\"".to_string(), "\"\"\"".to_string()),
                ],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // JavaScript/TypeScript
        self.add_language(
            "javascript".to_string(),
            Language {
                name: "JavaScript".to_string(),
                extensions: vec!["js".to_string(), "jsx".to_string(), "mjs".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        self.add_language(
            "typescript".to_string(),
            Language {
                name: "TypeScript".to_string(),
                extensions: vec!["ts".to_string(), "tsx".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // Java
        self.add_language(
            "java".to_string(),
            Language {
                name: "Java".to_string(),
                extensions: vec!["java".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // Go
        self.add_language(
            "go".to_string(),
            Language {
                name: "Go".to_string(),
                extensions: vec!["go".to_string()],
                single_line_comment: vec!["//".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // Ruby
        self.add_language(
            "ruby".to_string(),
            Language {
                name: "Ruby".to_string(),
                extensions: vec!["rb".to_string()],
                single_line_comment: vec!["#".to_string()],
                multi_line_comment: vec![("=begin".to_string(), "=end".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // Shell
        self.add_language(
            "shell".to_string(),
            Language {
                name: "Shell".to_string(),
                extensions: vec!["sh".to_string(), "bash".to_string(), "zsh".to_string()],
                single_line_comment: vec!["#".to_string()],
                multi_line_comment: vec![],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // SQL
        self.add_language(
            "sql".to_string(),
            Language {
                name: "SQL".to_string(),
                extensions: vec!["sql".to_string()],
                single_line_comment: vec!["--".to_string()],
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // HTML
        self.add_language(
            "html".to_string(),
            Language {
                name: "HTML".to_string(),
                extensions: vec!["html".to_string(), "htm".to_string()],
                single_line_comment: vec![],
                multi_line_comment: vec![("<!--".to_string(), "-->".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // CSS
        self.add_language(
            "css".to_string(),
            Language {
                name: "CSS".to_string(),
                extensions: vec!["css".to_string(), "scss".to_string(), "sass".to_string()],
                single_line_comment: vec!["//".to_string()], // For SCSS/SASS
                multi_line_comment: vec![("/*".to_string(), "*/".to_string())],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // YAML
        self.add_language(
            "yaml".to_string(),
            Language {
                name: "YAML".to_string(),
                extensions: vec!["yaml".to_string(), "yml".to_string()],
                single_line_comment: vec!["#".to_string()],
                multi_line_comment: vec![],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );

        // TOML
        self.add_language(
            "toml".to_string(),
            Language {
                name: "TOML".to_string(),
                extensions: vec!["toml".to_string()],
                single_line_comment: vec!["#".to_string()],
                multi_line_comment: vec![],
                nested_comments: false,
                preprocessor_prefix: None,
            },
        );
    }
}

/// Comment parser for handling single and multi-line comments
pub struct CommentParser {
    language: Language,
    ignore_preprocessor: bool,
}

impl CommentParser {
    pub fn new(language: Language, ignore_preprocessor: bool) -> Self {
        Self {
            language,
            ignore_preprocessor,
        }
    }

    /// REQ-4.2, REQ-4.4: Identify comments and mixed lines
    pub fn parse_line(&self, line: &str) -> LineType {
        let trimmed = line.trim();

        // REQ-4.5: Handle preprocessor directives
        if self.ignore_preprocessor {
            if let Some(prefix) = &self.language.preprocessor_prefix {
                if trimmed.starts_with(prefix) {
                    return LineType::Empty;
                }
            }
        }

        // Check if line is empty or whitespace
        if trimmed.is_empty() {
            return LineType::Empty;
        }

        // Check for single-line comments
        for prefix in &self.language.single_line_comment {
            if trimmed.starts_with(prefix) {
                // Check if comment contains only whitespace
                let comment_content = trimmed[prefix.len()..].trim();
                if comment_content.is_empty() {
                    return LineType::Empty;
                }
                return LineType::Comment;
            }
        }

        // Check if line contains both code and comments (REQ-4.4)
        for prefix in &self.language.single_line_comment {
            if line.contains(prefix) && !line.trim().starts_with(prefix) {
                return LineType::Mixed;
            }
        }

        // If we reach here, it's a logical line
        LineType::Logical
    }

    /// REQ-4.3: Handle nested comments
    pub fn is_in_multiline_comment(
        &self,
        line: &str,
        in_comment: &mut bool,
        depth: &mut usize,
    ) -> bool {
        if self.language.multi_line_comment.is_empty() {
            return false;
        }

        let mut line_copy = line.to_string();
        let mut result = *in_comment;

        for (start, end) in &self.language.multi_line_comment {
            if self.language.nested_comments {
                // Handle nested comments (REQ-4.3)
                while line_copy.contains(start) || line_copy.contains(end) {
                    if let Some(start_pos) = line_copy.find(start) {
                        if let Some(end_pos) = line_copy.find(end) {
                            if start_pos < end_pos {
                                *depth += 1;
                                line_copy = line_copy[start_pos + start.len()..].to_string();
                            } else {
                                if *depth > 0 {
                                    *depth -= 1;
                                }
                                line_copy = line_copy[end_pos + end.len()..].to_string();
                            }
                        } else {
                            *depth += 1;
                            line_copy = line_copy[start_pos + start.len()..].to_string();
                        }
                    } else if let Some(end_pos) = line_copy.find(end) {
                        if *depth > 0 {
                            *depth -= 1;
                        }
                        line_copy = line_copy[end_pos + end.len()..].to_string();
                    } else {
                        break;
                    }
                }
                result = *depth > 0;
            } else {
                // Simple multi-line comments
                if *in_comment {
                    if line.contains(end) {
                        *in_comment = false;
                        // Check if there's code after comment end
                        if let Some(pos) = line.find(end) {
                            let after = line[pos + end.len()..].trim();
                            if !after.is_empty() {
                                return false; // Mixed line
                            }
                        }
                    }
                    result = true;
                } else if line.contains(start) {
                    *in_comment = true;
                    // Check if comment closes on same line
                    if let Some(start_pos) = line.find(start) {
                        let after_start = &line[start_pos + start.len()..];
                        if after_start.contains(end) {
                            *in_comment = false;
                            // Check for code before or after
                            let before = line[..start_pos].trim();
                            if let Some(end_pos) = after_start.find(end) {
                                let after = after_start[end_pos + end.len()..].trim();
                                if !before.is_empty() || !after.is_empty() {
                                    return false; // Mixed line
                                }
                            }
                        }
                    }
                    result = true;
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineType {
    Empty,
    Comment,
    Logical,
    Mixed, // REQ-4.4: Lines with both code and comments
}
