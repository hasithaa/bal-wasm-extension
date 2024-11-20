use crate::parser::ast::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub sort_text: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Function,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Type,
    Keyword,
    Service,
}

pub struct CompletionContext {
    pub line: u32,
    pub character: u32,
    pub line_text: String,
    pub ast: ModuleNode,
    pub symbols: SymbolTable,
}

#[derive(Default, Clone)]
pub struct SymbolTable {
    pub functions: Vec<FunctionNode>,
    pub services: Vec<ServiceNode>,
    pub variables: Vec<VariableNode>,
    pub types: Vec<TypeNode>,
}

pub struct CompletionProvider {
    keywords: Vec<&'static str>,
}

impl CompletionProvider {
    pub fn new() -> Self {
        CompletionProvider {
            keywords: vec![
                "function", "service", "resource", "type", "record", "object",
                "public", "private", "return", "if", "else", "while", "foreach",
                "true", "false", "null", "import",
            ],
        }
    }

    pub fn get_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Determine completion context
        if self.is_type_context(&context.line_text, context.character) {
            completions.extend(self.get_type_completions(&context.symbols));
        } else if self.is_function_context(&context.line_text, context.character) {
            completions.extend(self.get_function_completions(&context.symbols));
        } else if self.is_module_context(&context.line_text, context.character) {
            completions.extend(self.get_module_completions());
        }

        // Always add keyword suggestions if at start of line or after whitespace
        if self.should_show_keywords(&context.line_text, context.character) {
            completions.extend(self.get_keyword_completions());
        }

        completions
    }

    fn is_type_context(&self, line: &str, position: u32) -> bool {
        let prefix = &line[..position as usize];
        prefix.trim_end().ends_with(':')
            || prefix.trim_end().ends_with("type")
            || prefix.trim_end().ends_with("record")
    }

    fn is_function_context(&self, line: &str, position: u32) -> bool {
        let prefix = &line[..position as usize];
        prefix.trim_end().ends_with('.')
    }

    fn is_module_context(&self, line: &str, position: u32) -> bool {
        let prefix = &line[..position as usize];
        prefix.trim_end().ends_with("import")
    }

    fn should_show_keywords(&self, line: &str, position: u32) -> bool {
        let prefix = &line[..position as usize];
        prefix.trim().is_empty() || prefix.ends_with(' ')
    }

    fn get_type_completions(&self, symbols: &SymbolTable) -> Vec<CompletionItem> {
        let mut completions = vec![
            CompletionItem {
                label: "string".to_string(),
                kind: CompletionItemKind::Type,
                detail: Some("String type".to_string()),
                documentation: Some("Represents a string value".to_string()),
                sort_text: None,
                insert_text: None,
            },
            CompletionItem {
                label: "int".to_string(),
                kind: CompletionItemKind::Type,
                detail: Some("Integer type".to_string()),
                documentation: Some("Represents an integer value".to_string()),
                sort_text: None,
                insert_text: None,
            },
            CompletionItem {
                label: "boolean".to_string(),
                kind: CompletionItemKind::Type,
                detail: Some("Boolean type".to_string()),
                documentation: Some("Represents a boolean value".to_string()),
                sort_text: None,
                insert_text: None,
            },
        ];

        // Add user-defined types from symbol table
        for type_node in &symbols.types {
            completions.push(CompletionItem {
                label: type_node.type_name.clone(),
                kind: CompletionItemKind::Type,
                detail: Some("User-defined type".to_string()),
                documentation: None,
                sort_text: None,
                insert_text: None,
            });
        }

        completions
    }

    fn get_function_completions(&self, symbols: &SymbolTable) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        for function in &symbols.functions {
            let mut detail = String::new();
            // Create function signature
            detail.push_str("function ");
            detail.push_str(&function.name);
            detail.push('(');
            
            let params: Vec<String> = function.params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.param_type.type_name))
                .collect();
            detail.push_str(&params.join(", "));
            detail.push(')');

            if let Some(return_type) = &function.return_type {
                detail.push_str(" returns ");
                detail.push_str(&return_type.type_name);
            }

            completions.push(CompletionItem {
                label: function.name.clone(),
                kind: CompletionItemKind::Function,
                detail: Some(detail),
                documentation: None,
                sort_text: None,
                insert_text: Some(format!("{}()", function.name)),
            });
        }

        completions
    }

    fn get_module_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "ballerina/http".to_string(),
                kind: CompletionItemKind::Module,
                detail: Some("HTTP module".to_string()),
                documentation: Some("Provides HTTP client and server implementations".to_string()),
                sort_text: None,
                insert_text: None,
            },
            CompletionItem {
                label: "ballerina/io".to_string(),
                kind: CompletionItemKind::Module,
                detail: Some("I/O module".to_string()),
                documentation: Some("Provides I/O operations".to_string()),
                sort_text: None,
                insert_text: None,
            },
        ]
    }

    fn get_keyword_completions(&self) -> Vec<CompletionItem> {
        self.keywords
            .iter()
            .map(|&keyword| CompletionItem {
                label: keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: None,
                documentation: None,
                sort_text: None,
                insert_text: None,
            })
            .collect()
    }
} 