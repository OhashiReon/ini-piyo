use crate::parser::{IniLine, parse_ini_lines};
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
enum SectionType {
    Global,
    Named(String),
}

#[derive(Clone)]
struct Ast {
    sections: HashMap<SectionType, Vec<IniLine>>,
}

impl Ast {
    fn new(text: &str) -> Self {
        let mut sections: HashMap<SectionType, Vec<IniLine>> = HashMap::new();
        let mut current_section = SectionType::Global;

        for line in parse_ini_lines(text) {
            if let IniLine::Section { name, .. } = &line {
                current_section = SectionType::Named(name.clone());
            }

            sections
                .entry(current_section.clone())
                .or_default()
                .push(line);
        }
        Ast { sections }
    }

    fn get_section(&self, section: &SectionType) -> Option<IniLine> {
        match section {
            SectionType::Global => None,
            SectionType::Named(_) => {
                self.sections.get(section).and_then(|lines| lines.first().cloned())
            }
        }
    }

    fn get_key(&self, section: &SectionType, target_key: &str) -> Option<IniLine> {
        if let Some(lines) = self.sections.get(section) {
            for line in lines {
                if let IniLine::KeyValue { key, .. } = line {
                    if key == target_key {
                        return Some(line.clone());
                    }
                }
            }
        }
        None
    }

    fn get_other(&self, section: &SectionType, target_raw: &str) -> Option<IniLine> {
        if let Some(lines) = self.sections.get(section) {
            for line in lines {
                if let IniLine::Other { raw } = line {
                    if raw == target_raw {
                        return Some(line.clone());
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct DiffResult {
    pub target: Option<IniLine>,
    pub base: IniLine,
}

pub fn diff(base_text: &str, target_text: &str) -> Vec<DiffResult> {
    let target_ast = Ast::new(target_text);
    let base_lines = parse_ini_lines(base_text);
    let mut current_section = SectionType::Global;
    let mut result = Vec::with_capacity(base_lines.len());

    for line in base_lines {
        let (base_line, target_line) = match &line {
            IniLine::Section { name, .. } => {
                current_section = SectionType::Named(name.clone());
                let target_line = target_ast.get_section(&current_section);
                (line.clone(), target_line)
            }
            IniLine::KeyValue { key, .. } => {
                let target_line = target_ast.get_key(&current_section, key);
                (line.clone(), target_line)
            }
            IniLine::Other { raw } => {
                let target_line = target_ast.get_other(&current_section, raw);
                (line.clone(), target_line)
            }
        };
        result.push(DiffResult {
            target: target_line,
            base: base_line,
        });
    }
    result
}