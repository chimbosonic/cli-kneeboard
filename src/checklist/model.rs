use log::{debug, warn};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::error;
use toml::Table;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Checklist {
    pub items: Vec<ChecklistItem>,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub text: String,
    pub optional: bool,
    pub resolved: bool,
}

impl Default for ChecklistItem {
    fn default() -> Self {
        ChecklistItem {
            text: "".to_string(),
            optional: false,
            resolved: false,
        }
    }
}

impl Checklist {
    pub fn from_markdown(markdown_input: String) -> Result<Checklist> {
        let mut checklist = Checklist {
            name: "".to_string(),
            items: Vec::new(),
        };
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(&markdown_input, options);
        let (mut is_list, mut is_checklist, mut is_list_item) = (false, false, false);
        let mut checklist_item = ChecklistItem::default();
        for event in parser {
            match &event {
                Event::Start(tag) => match *tag {
                    Tag::List(_) => is_list = true,
                    Tag::Item => {
                        if is_list && is_checklist && is_list_item {
                            debug!(
                                "[extract_checklist] Adding ChecklistItem: {:?}",
                                checklist_item
                            );
                            checklist.items.push(checklist_item.clone());

                            checklist_item = ChecklistItem::default();
                        }
                        is_list_item = true;
                    }
                    _ => (),
                },
                Event::Text(s) => {
                    if is_list && is_checklist && is_list_item {
                        debug!("[extract_checklist] ChecklistItem Found text: {:?}", s);
                        checklist_item.text.push_str(s);
                    }
                }
                Event::End(tag) => match *tag {
                    TagEnd::List(_) => is_list = false,
                    TagEnd::Item => {
                        is_list_item = false;
                        if is_list && is_checklist {
                            debug!(
                                "[extract_checklist] Adding ChecklistItem: {:?}",
                                checklist_item
                            );
                            checklist.items.push(checklist_item.clone());

                            checklist_item = ChecklistItem::default();
                        }
                    }
                    _ => (),
                },
                Event::Html(s) => {
                    if s.contains("checklist") && s.contains("<!--") {
                        checklist.name = extract_checklist_name(s.to_string());
                        is_checklist ^= true
                    }
                }
                _ => (),
            };
        }

        if checklist.items.is_empty() {
            warn!("[extract_checklist] Found No Checklist or and Items returning Empty Checklist");
            return Err(
                "[extract_checklist] Found No Checklist or and Items returning Empty Checklist"
                    .into(),
            );
        }
        for checklist_item in checklist.items.iter_mut() {
            if checklist_item.text.contains("[OPTIONAL]") {
                debug!(
                    "[set_optionality] Setting {:?} to optional",
                    checklist_item.text
                );
                checklist_item.optional = true;
            }
        }

        Ok(checklist)
    }

    pub fn to_toml(&self) -> Result<String> {
        match toml::to_string_pretty(self) {
            Ok(s) => Ok(s),
            Err(_) => Err("[to_toml] failed to generate toml".into()),
        }
    }

    pub fn from_toml(input_string: String) -> Result<Checklist> {
        match toml::from_str::<Checklist>(&input_string) {
            Ok(checklist) => Ok(checklist),
            Err(_) => Err("[from_toml] failed parse ChecklistItems from TOML".into()),
        }
    }

    pub fn get_count_unresolved(&self) -> usize {
        let mut count: usize = 0;
        for checklist_item in &self.items {
            if !&checklist_item.resolved && !&checklist_item.optional {
                count += 1;
            }
        }
        count
    }
}

pub(super) fn extract_checklist_name(input_string: String) -> String {
    debug!(
        "[extract_checklist_name] Extracting name from : {:?}",
        input_string
    );
    let mut name = String::from("");
    // Remove HTML comment brackets
    let input_string = input_string.replace(&['<', '!', '-', '>'][..], "");
    let input_string = input_string.trim();
    let result = input_string.parse::<Table>();
    match result {
        Ok(value) => match value.get("checklist") {
            Some(val) => {
                debug!("[extract_checklist_name] Found: {:?}", val);
                name = val.as_str().unwrap().to_string();
            }
            None => {
                debug!("[extract_checklist_name] Found some TOML but name wasn't following spec");
            }
        },
        Err(_err) => {
            debug!("[extract_checklist_name] No Name found");
        }
    }

    if name.is_empty() {
        name = String::from("checklist");
    }

    name
}
