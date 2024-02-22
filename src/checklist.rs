use log::{debug, warn};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde_derive::{Deserialize, Serialize};
use std::error;
use toml::Value;

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

    //This will return a u8 and is allowed to overflow this is because we use it as a ExitCode which has to be a u8
    pub fn get_count_unresolved(&self) -> u8 {
        let mut count: u8 = 0;
        for checklist_item in &self.items {
            if !&checklist_item.resolved && !&checklist_item.optional {
                count = count.wrapping_add(1);
            }
        }
        count
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

    name
}

// Unit Tests all internal functions must be tested here. At least one test per function unless impossible
#[cfg(test)]
mod tests {
    // use log::Level;

    use super::*;

    fn generate_test_checklist(count: u128, name: String, optional: Option<bool>) -> Checklist {
        let mut test_checklist = Checklist {
            items: Vec::<ChecklistItem>::new(),
            name: name.clone(),
        };

        for i in 0..count {
            test_checklist.items.push(ChecklistItem {
                text: format!("{} item {:}", &name, i),
                optional: optional.unwrap_or(false),
                resolved: false,
            })
        }

        test_checklist
    }

    // Checklist Tests
    #[test_log::test]
    fn create_new_checklist_from_markdown_simple_single_item() {
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] test checklist item
        "#;
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
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

    #[test_log::test]
    fn create_new_checklist_from_markdown_simple_single_optional_item() {
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] test checklist item [OPTIONAL]
        "#;
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
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

    #[test_log::test]
    fn create_new_checklist_from_markdown_simple_multiple_items() {
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist - name -->
- [x] test checklist item 1
- [x] test checklist item 2
        "#;
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
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

    #[test_log::test]
    fn create_new_checklist_from_markdown_mixed_lists_multiple_items() {
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
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
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

    #[test_log::test]
    fn create_new_checklist_from_markdown_with_nested_items() {
        let markdown_input = r#"
<!-- checklist -->
- [ ] test checklist item
    - [ ] test checklist nested item
<!-- checklist -->
        "#;
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
        let mut test_checklist = Checklist {
            name: "".to_string(),
            items: Vec::new(),
        };
        test_checklist.items.push(ChecklistItem {
            text: "test checklist item".to_string(),
            optional: false,
            resolved: false,
        });
        test_checklist.items.push(ChecklistItem {
            text: "test checklist nested item".to_string(),
            optional: false,
            resolved: false,
        });
        assert_eq!(test_checklist.items, checklist.items)
    }

    #[test_log::test]
    fn create_new_checklist_from_markdown_single_item_containing_markdown_formating() {
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist -->
- [x] Example paragraph with **lorem** _ipsum_ text.
        "#;
        let checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
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

    #[test_log::test]
    #[should_panic]
    fn create_new_checklist_from_markdown_no_checklist() {
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
- [x] test not checklist item 1
- [x] test not checklist item 2
- [x] test not checklist item 3
- [x] test not checklist item 4
        "#;
        let _checklist = Checklist::from_markdown(String::from(markdown_input)).unwrap();
    }

    #[test_log::test]
    fn save_and_load_checklist() {
        let mut test_checklist = Checklist {
            name: "test_checklist".to_string(),
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
        let toml_string = test_checklist.to_toml().unwrap();
        assert_eq!(toml_string,"name = \"test_checklist\"\n\n[[items]]\ntext = \"test checklist item 1\"\noptional = false\nresolved = false\n\n[[items]]\ntext = \"test checklist item 2\"\noptional = false\nresolved = false\n".to_string());

        let reconstructed_checklist = Checklist::from_toml(toml_string).unwrap();
        assert_eq!(reconstructed_checklist.items, test_checklist.items);
    }

    #[test_log::test]
    fn generate_test_checklist_test() {
        let test_checklist = generate_test_checklist(300, "test checklist".to_string(), None);
        assert_eq!(test_checklist.items.len(), 300);
    }

    #[test_log::test]
    fn count_unresolved_checklist() {
        let test_checklist = generate_test_checklist(300, "test checklist".to_string(), None);
        assert_eq!(test_checklist.items.len(), 300);
        assert_eq!(test_checklist.get_count_unresolved(), 44);
    }

    #[test_log::test]
    fn count_unresolved_optional_checklist() {
        let test_checklist = generate_test_checklist(300, "test checklist".to_string(), Some(true));
        assert_eq!(test_checklist.items.len(), 300);
        assert_eq!(test_checklist.get_count_unresolved(), 0);
    }

    #[test_log::test]
    #[should_panic]
    fn load_non_checklist() {
        let _checklist = Checklist::from_toml("nottoml".to_string()).unwrap();
    }

    // extract_checklist_name Tests
    #[test_log::test]
    fn extract_checklist_name_no_name() {
        let markdown_input = "<!-- checklist -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("checklist", checklist_name)
    }

    #[test_log::test]
    fn extract_checklist_name_name_following_spec() {
        let markdown_input = "<!-- checklist = 'test_name' -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("test_name", checklist_name)
    }

    #[test_log::test]
    fn extract_checklist_name_name_not_following_spec() {
        let markdown_input = "<!-- blah = 'test_name' -->";
        let checklist_name = extract_checklist_name(String::from(markdown_input));
        assert_eq!("checklist", checklist_name)
    }
}
