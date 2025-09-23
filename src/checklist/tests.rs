use crate::checklist::model::extract_checklist_name;

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
