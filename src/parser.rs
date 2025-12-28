#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    Equals, // =
    Colon,  // :
}

impl Separator {
    pub fn as_str(&self) -> &str {
        match self {
            Separator::Equals => "=",
            Separator::Colon => ":",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IniLine {
    Section {
        raw: String,
        name: String,
    },

    KeyValue {
        raw: String,
        key: String,
        separator: Separator,
        value: String,
    },

    Other {
        raw: String,
    },
}

impl IniLine {
    pub fn get_raw(&self) -> &str {
        match self {
            IniLine::Section { raw, .. } => raw,
            IniLine::KeyValue { raw, .. } => raw,
            IniLine::Other { raw } => raw,
        }
    }
}

fn split_key_value(s: &str) -> Option<(&str, (Separator, &str))> {
    if let Some(idx) = s.find('=') {
        let (k, v) = s.split_at(idx);
        return Some((k, (Separator::Equals, &v[1..])));
    }

    if let Some(idx) = s.find(':') {
        let (k, v) = s.split_at(idx);
        return Some((k, (Separator::Colon, &v[1..])));
    }
    None
}

fn parse_line(line: &str) -> IniLine {
    let raw = line.to_string();
    let trimmed = line.trim_start();

    if trimmed.starts_with('[') {
        if let Some(end) = trimmed.find(']') {
            let name = &trimmed[1..end];
            return IniLine::Section {
                raw,
                name: name.to_string(),
            };
        } else {
            return IniLine::Other { raw };
        }
    }
    if let Some((key, value)) = split_key_value(trimmed) {
        return IniLine::KeyValue {
            raw,
            key: key.to_string(),
            separator: value.0,
            value: value.1.to_string(),
        };
    }

    IniLine::Other { raw }
}

pub fn parse_ini_lines(input: &str) -> Vec<IniLine> {
    input.lines().map(parse_line).collect()
}