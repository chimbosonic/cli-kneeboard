use pulldown_cmark::{Event,Parser, Options, Tag};
use toml::Value;


pub struct Checklist {
    list: Vec<ChecklistItem>,
    name: String
}

#[derive(Debug,PartialEq)]
pub struct ChecklistItem {
    text: String,
    optional: bool,
    resolved: bool
}

// TODO: extract optional items, extract checklist name, Create Struct representing a Checklist and its items and Items with their text, status and optionality
pub fn extract_checklist(markdown_input: String) -> Checklist {
    // println!("\nParsing the following markdown string:\n#####\n{}\n#####\n", markdown_input);
    let mut checklist = Checklist {
        name: "".to_string(),
        list: Vec::new()
    };
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut is_list = false;
    let mut is_check_list = false;
    for event in parser {
        match &event {
            Event::Start(tag) => {
                match tag {                 
                    Tag::List(_) => { is_list = true },
                    _ => ()
                }},
            Event::Text(s) => {
                if is_list && is_check_list {
                    checklist.list.push(ChecklistItem {
                        text: s.to_string(),
                        optional: false,
                        resolved: false
                    });
                }
            },
            Event::End(tag) => {
                match tag {      
                    Tag::List(_) => { is_list = false },           
                    _ => ()
                }},
            Event::Html(s) => {
                if s.contains("checklist") && s.contains("<!--") {
                    // let s = s.replace(&['<', '!', '-', '>'][..], "");
                    // let result = s.parse::<Value>();
                    // // TODO: implement name extraction
                    // match result {
                    //     Ok(value) => {
                    //         match value["checklist"].as_str() {
                    //             Some(val) => {
                    //                 println!("{}",val);
                    //             },
                    //             None => {
                    //             }
                    //         }
                    //     },
                    //     Err(_err) => {
                    //         println!("No Name found");
                    //     }
                    // }
                    is_check_list ^= true
                }
            },
            _ => ()
        };
    }

    return checklist
}


// Unit Tests all internal functions must be tested here. At least one test per function unless impossible
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_checklist_from_markdown_simple_single_item(){
        let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.\n<!-- checklist = 'test' -->\n - [x] test checklist item";
        let check_list = extract_checklist(String::from(markdown_input));
        let mut test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        test_list.list.push(ChecklistItem {
            text: "test checklist item".to_string(),
            optional: false,
            resolved: false
        });
        assert_eq!(test_list.list,check_list.list)
    }

    #[test]
    fn extract_checklist_from_markdown_no_name(){
        let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.\n<!-- checklist -->\n - [x] test checklist item";
        let check_list = extract_checklist(String::from(markdown_input));
        let mut test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        test_list.list.push(ChecklistItem {
            text: "test checklist item".to_string(),
            optional: false,
            resolved: false
        });
        assert_eq!(test_list.list,check_list.list)
    }

    #[test]
    fn extract_checklist_from_markdown_name_not_following_spec(){
        let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.\n<!-- wrongkey = 'test' -->\n - [x] test checklist item";
        let check_list = extract_checklist(String::from(markdown_input));
        let mut test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        assert_eq!(test_list.list,check_list.list)
    }

    #[test]
    fn extract_checklist_from_markdown_simple_multiple_items(){
        let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.\n<!-- checklist = 'test' -->\n - [x] test checklist item 1\n- [x] test checklist item 2";
        let check_list = extract_checklist(String::from(markdown_input));
        let mut test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        test_list.list.push(ChecklistItem {
            text: "test checklist item 1".to_string(),
            optional: false,
            resolved: false
        });
        test_list.list.push(ChecklistItem {
            text: "test checklist item 2".to_string(),
            optional: false,
            resolved: false
        });
        assert_eq!(test_list.list,check_list.list)
    }

    #[test]
    fn extract_checklist_from_markdown_mixed_lists_multiple_items(){
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
<!-- checklist - name -->
- [x] test checklist item 1
- [x] test checklist item 2
<!-- checklist - name -->
- [x] test not checklist item 1
- [x] test not checklist item 2
        "#;
        let check_list = extract_checklist(String::from(markdown_input));
        let mut test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        test_list.list.push(ChecklistItem {
            text: "test checklist item 1".to_string(),
            optional: false,
            resolved: false
        });
        test_list.list.push(ChecklistItem {
            text: "test checklist item 2".to_string(),
            optional: false,
            resolved: false
        });
        assert_eq!(test_list.list,check_list.list)
    }

    #[test]
    fn extract_checklist_from_markdown_no_checklist(){
        let markdown_input = r#"
# Example Heading
Example paragraph with **lorem** _ipsum_ text.
- [x] test not checklist item 1
- [x] test not checklist item 2
- [x] test not checklist item 3
- [x] test not checklist item 4
        "#;
        let check_list = extract_checklist(String::from(markdown_input));
        let test_list = Checklist {
            name: "".to_string(),
            list: Vec::new()
        };
        assert_eq!(test_list.list,check_list.list)
    }
}