use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

fn main() {
    // create root cursive object.
    let mut siv = cursive::default();

    // customize the views

    // add a select view
    let select = SelectView::<String>::new()
        .on_submit(on_submit) // callback function when a name is selected
        .with_name("select") // id to identify the view
        .fixed_size((10, 5)); // size when initial list is empty

    // add buttons layout
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add))
        .child(Button::new("Remove", remove))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    // arrange the elements
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a profile!"),
    );

    // global listerner to exit from program.
    siv.add_global_callback('q', |s| s.quit());

    // run the cursive object
    siv.run();
}

fn add(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView| view.add_item_str(name));
        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(
            EditView::new().on_submit(ok).with_name("name"), // .fixed_width(10),
        )
        .title("Add new contact")
        // another callback if we add using ok button
        .button("Ok", |s| {
            // find the current name entered in editview
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Exit", |s| {
            s.pop_layer();
        }),
    );
}

fn remove(s: &mut Cursive) {
    // find_name returns a handler through which we can mutate the view.
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        // get the id of selected element
        None => s.add_layer(Dialog::info("No contact to remove.")), // if empty list
        Some(focus) => {
            // focus is var name for id here
            select.remove_item(focus);
        }
    }
}

/// Shows the details associated with a contact.
fn on_submit(s: &mut Cursive, name: &str) {
    // s.pop_layer();
    s.add_layer(
        Dialog::text(format!("Name: {}\nAwesome: Yes", name))
            .title(format!("{}'s Info", name))
            .button("go back", |s| {
                s.pop_layer();
            })
            .button("Quit!", |s| s.quit()),
    );
}
