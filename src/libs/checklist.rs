use log::{debug, error, info, warn};
use pulldown_cmark::{Event, Options, Parser, Tag};
use toml::Value;

#[derive(Clone)]
pub struct Checklist {
    items: Vec<ChecklistItem>,
    name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChecklistItem {
    text: String,
    optional: bool,
    resolved: bool,
}

pub fn extract_checklist(markdown_input: String) -> Checklist {
    let mut checklist = Checklist {
        name: "".to_string(),
        items: Vec::new(),
    };
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut is_list = false;
    let mut is_checklist = false;
    let mut is_list_item = false;
    let mut checklist_item = ChecklistItem {
        text: "".to_string(),
        optional: false,
        resolved: false,
    };
    for event in parser {
        match &event {
            Event::Start(tag) => match tag {
                Tag::List(_) => is_list = true,
                Tag::Item => is_list_item = true,
                _ => (),
            },
            Event::Text(s) => {
                if is_list && is_checklist && is_list_item {
                    debug!("[extract_checklist] ChecklistItem Found text: {:?}", s);
                    checklist_item.text.push_str(s);
                }
            }
            Event::End(tag) => match tag {
                Tag::List(_) => is_list = false,
                Tag::Item => {
                    is_list_item = false;
                    if is_list && is_checklist {
                        debug!(
                            "[extract_checklist] Adding ChecklistItem: {:?}",
                            checklist_item
                        );
                        checklist.items.push(checklist_item.clone());

                        checklist_item = ChecklistItem {
                            text: "".to_string(),
                            optional: false,
                            resolved: false,
                        }
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

    if checklist.items.len() == 0 {
        warn!("[extract_checklist] Found No Checklist or and Items returning Empty Checklist")
    }

    checklist = checklist.set_optionality();
    return checklist;
}

impl Checklist {
    fn set_optionality(mut self) -> Checklist {
        for checklist_item in self.items.iter_mut() {
            if checklist_item.text.contains("[OPTIONAL]") {
                debug!(
                    "[set_optionality] Setting {:?} to optional",
                    checklist_item.text
                );
                checklist_item.optional = true;
            }
        }

        return self;
    }
}
fn extract_checklist_name(input_string: String) -> String {
    debug!(
        "[extract_checklist_name] Extracting name from : {:?}",
        input_string
    );
    let mut name = String::from("");
    let input_string = input_string.replace(&['<', '!', '-', '>'][..], ""); // Remove HTML comment brackets
    let result = input_string.parse::<Value>();
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

    return name;
}

// Unit Tests all internal functions must be tested here. At least one test per function unless impossible
#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs::logger::setup_logger;
    use log::LevelFilter;

    fn setup() {
        setup_logger(LevelFilter::Info) // Change this to Debug if you need debug logs
    }

    // extract_checklist Tests
    #[test]
    fn extract_checklist_from_markdown_simple_single_item() {
        setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] test checklist item
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let mut test_checklist = Checklist {
            name: "checklist".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item".to_string(),
            optional: false,
            resolved: false,
        });
        assert_eq!(test_checklist.name, checklist.name);
        assert_eq!(test_checklist.items, checklist.items)
    }

    #[test]
    fn extract_checklist_from_markdown_simple_single_optional_item() {
        setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] test checklist item [OPTIONAL]
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let mut test_checklist = Checklist {
            name: "checklist".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item [OPTIONAL]".to_string(),
            optional: true,
            resolved: false,
        });
        assert_eq!(test_checklist.name, checklist.name);
        assert_eq!(test_checklist.items, checklist.items);
    }

    #[test]
    fn extract_checklist_from_markdown_simple_multiple_items() {
        setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist - name -->
- [x] test checklist item 1
- [x] test checklist item 2
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let mut test_checklist = Checklist {
            name: "checklist".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item 1".to_string(),
            optional: false,
            resolved: false,
        });
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item 2".to_string(),
            optional: false,
            resolved: false,
        });
        assert_eq!(test_checklist.name, checklist.name);
        assert_eq!(test_checklist.items, checklist.items)
    }

    #[test]
    fn extract_checklist_from_markdown_mixed_lists_multiple_items() {
        setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] test checklist item 1
- [x] test checklist item 2
<!-- checklist -->
- [x] test not checklist item 1
- [x] test not checklist item 2
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let mut test_checklist = Checklist {
            name: "".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item 1".to_string(),
            optional: false,
            resolved: false,
        });
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item 2".to_string(),
            optional: false,
            resolved: false,
        });
        assert_eq!(test_checklist.items, checklist.items)
    }

    #[test]
    fn extract_checklist_from_markdown_single_item_containing_markdown_formating() {
        setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] Example paragraph with **lorem** _ipsum_ text.
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let mut test_checklist = Checklist {
            name: "checklist".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "Example paragraph with lorem ipsum text.".to_string(),
            optional: false,
            resolved: false,
        });
        assert_eq!(test_checklist.items, checklist.items)
    }

    #[test]
    fn extract_checklist_from_markdown_no_checklist() {
        // setup();
        testing_logger::setup();
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
- [x] test not checklist item 1
- [x] test not checklist item 2
- [x] test not checklist item 3
- [x] test not checklist item 4
        "#;
        let checklist = extract_checklist(String::from(markdown_input));
        let test_checklist = Checklist {
            name: "checklist".to_string(),
            items: Vec::new(),
        };
        assert_eq!(test_checklist.items, checklist.items);

        testing_logger::validate(|captured_logs| {
            let warnings = captured_logs
                .iter()
                .filter(|c| c.level == log::Level::Warn)
                .collect::<Vec<&testing_logger::CapturedLog>>();

            assert_eq!(warnings.len(), 1);
            assert_eq!(
                warnings[0].body,
                "[extract_checklist] Found No Checklist or and Items returning Empty Checklist"
            );
        });
    }

    // extract_checklist_name Tests
    #[test]
    fn extract_checklist_name_no_name() {
        setup();
        let markdown_input = "<!-- checklist -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("checklist", checklist_name)
    }

    #[test]
    fn extract_checklist_name_name_following_spec() {
        setup();
        let markdown_input = "<!-- checklist = 'test_name' -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("test_name", checklist_name)
    }

    #[test]
    fn extract_checklist_name_name_not_following_spec() {
        setup();
        let markdown_input = "<!-- blah = 'test_name' -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("checklist", checklist_name)
    }
}
