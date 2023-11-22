use crate::checklist::Checklist;
use crate::checklist::ChecklistItem;
use cursive::theme::{BorderStyle, Palette};
use cursive::traits::*;
use cursive::views::{NamedView, Panel, ViewRef};
use cursive::{
    view::Resizable,
    views::{Checkbox, ListView},
};

pub fn draw(checklist: Checklist) -> Checklist {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::*;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.dark();
            palette[TitlePrimary] = Green.light();
            palette[Secondary] = Green.light();
            palette[Highlight] = Green.dark();
        }),
    });

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('w', |s| s.quit());

    siv.add_global_callback('~', |s| s.toggle_debug_console());

    let mut checklist_view = ListView::new();

    for checklist_item in &checklist.items {
        if checklist_item.resolved {
            checklist_view.add_child(
                &checklist_item.text,
                NamedView::new(&checklist_item.text, Checkbox::new().checked()),
            )
        } else {
            checklist_view.add_child(
                &checklist_item.text,
                NamedView::new(&checklist_item.text, Checkbox::new()),
            )
        }
    }

    let mut main_panel = Panel::new(checklist_view.scrollable());

    main_panel.set_title(&checklist.name);
    // Creates a dialog with a single "Quit" button
    siv.add_fullscreen_layer(main_panel.full_width());

    // Starts the event loop.
    siv.run();

    let mut final_checklist = Checklist {
        items: Vec::<ChecklistItem>::new(),
        name: checklist.name.clone(),
    };

    for checklist_item in &checklist.items {
        let checkboxview: ViewRef<Checkbox> = siv.find_name(&checklist_item.text).unwrap();
        if checkboxview.is_checked() {
            final_checklist.items.push(ChecklistItem {
                text: checklist_item.text.clone(),
                optional: checklist_item.optional,
                resolved: true,
            })
        } else {
            final_checklist.items.push(ChecklistItem {
                text: checklist_item.text.clone(),
                optional: checklist_item.optional,
                resolved: false,
            })
        }
    }
    final_checklist
}
