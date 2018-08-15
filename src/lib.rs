#![recursion_limit="128"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::format::Json;
use yew::services::storage::{StorageService, Area};

const KEY: &'static str = "yew.rustconf_scheduler.self";

pub struct Model {
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
//    entries: Vec<Entry>,
//    filter: Filter,
//    value: String,
//    edit_value: String,
}

//#[derive(Serialize, Deserialize)]
//struct Entry {
//    description: String,
//    completed: bool,
//    editing: bool,
//}

pub enum Msg {
    Select(usize),
    Deselect(usize),
    DeselectAll(),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut storage = StorageService::new(Area::Local);
//        let entries = {
//            if let Json(Ok(restored_model)) = storage.restore(KEY) {
//                restored_model
//            } else {
//                Vec::new()
//            }
//        };
        let state = State {
//            entries,
//            filter: Filter::All,
//            value: "".into(),
//            edit_value: "".into(),
        };
        Model { storage, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        match msg {
//            Msg::Add => {
//                let entry = Entry {
//                    description: self.state.value.clone(),
//                    completed: false,
//                    editing: false,
//                };
//                self.state.entries.push(entry);
//                self.state.value = "".to_string();
//            }
//            Msg::Edit(idx) => {
//                let edit_value = self.state.edit_value.clone();
//                self.state.complete_edit(idx, edit_value);
//                self.state.edit_value = "".to_string();
//            }
//            Msg::Update(val) => {
//                println!("Input: {}", val);
//                self.state.value = val;
//            }
//            Msg::UpdateEdit(val) => {
//                println!("Input: {}", val);
//                self.state.edit_value = val;
//            }
//            Msg::Remove(idx) => {
//                self.state.remove(idx);
//            }
//            Msg::SetFilter(filter) => {
//                self.state.filter = filter;
//            }
//            Msg::ToggleEdit(idx) => {
//                self.state.edit_value = self.state.entries[idx].description.clone();
//                self.state.toggle_edit(idx);
//            }
//            Msg::ToggleAll => {
//                let status = !self.state.is_all_completed();
//                self.state.toggle_all(status);
//            }
//            Msg::Toggle(idx) => {
//                self.state.toggle(idx);
//            }
//            Msg::ClearCompleted => {
//                self.state.clear_completed();
//            }
//            Msg::Nope => {}
//        }
//        self.storage.store(KEY, Json(&self.state.entries));
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="schedule-table-wrapper",>
                <table>
                  <tr>
                    <td>{ "Month" }</td>
                    <td>{ "Savings" }</td>
                    <td>{ "Savings for holiday!" }</td>
                  </tr>
                  <tr>
                    <td>{ "January" }</td>
                    <td>{ "$100" }</td>
                    <td rowspan="2",>{ "$50" }</td>
                  </tr>
                  <tr>
                    <td>{ "February" }</td>
                    <td>{ "$80" }</td>
                  </tr>
                </table>
            </div>
        }
    }
}

impl Model {
//    fn view_filter(&self, filter: Filter) -> Html<Model> {
//        let flt = filter.clone();
//        html! {
//            <li>
//                <a class=if self.state.filter == flt { "selected" } else { "not-selected" },
//                   href=&flt,
//                   onclick=|_| Msg::SetFilter(flt.clone()),>
//                    { filter }
//                </a>
//            </li>
//        }
//    }
//
//    fn view_input(&self) -> Html<Model> {
//        html! {
//            // You can use standard Rust comments. One line:
//            // <li></li>
//            <input class="new-todo",
//                   placeholder="What needs to be done?",
//                   value=&self.state.value,
//                   oninput=|e| Msg::Update(e.value),
//                   onkeypress=|e| {
//                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
//                   }, />
//            /* Or multiline:
//            <ul>
//                <li></li>
//            </ul>
//            */
//        }
//    }
}

//fn view_entry((idx, entry): (usize, &Entry)) -> Html<Model> {
//    html! {
//        <li class=if entry.editing == true { "editing" } else { "" },>
//            <div class="view",>
//                <input class="toggle", type="checkbox", checked=entry.completed, onclick=|_| Msg::Toggle(idx), />
//                <label ondoubleclick=|_| Msg::ToggleEdit(idx),>{ &entry.description }</label>
//                <button class="destroy", onclick=|_| Msg::Remove(idx), />
//            </div>
//            { view_entry_edit_input((idx, &entry)) }
//        </li>
//    }
//}
//
//fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Html<Model> {
//    if entry.editing == true {
//        html! {
//            <input class="edit",
//                   type="text",
//                   value=&entry.description,
//                   oninput=|e| Msg::UpdateEdit(e.value),
//                   onblur=|_| Msg::Edit(idx),
//                   onkeypress=|e| {
//                      if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
//                   }, />
//        }
//    } else {
//        html! { <input type="hidden", /> }
//    }
//}
//

//#[derive(Clone, PartialEq)]
//#[derive(Serialize, Deserialize)]
//pub enum Filter {
//    All,
//    Active,
//    Completed,
//}

//impl<'a> Into<Href> for &'a Filter {
//    fn into(self) -> Href {
//        match *self {
//            Filter::All => "#/".into(),
//            Filter::Active => "#/active".into(),
//            Filter::Completed => "#/completed".into(),
//        }
//    }
//}
//
//impl Filter {
//    fn fit(&self, entry: &Entry) -> bool {
//        match *self {
//            Filter::All => true,
//            Filter::Active => !entry.completed,
//            Filter::Completed => entry.completed,
//        }
//    }
//}

//impl State {
//    fn total(&self) -> usize {
//        self.entries.len()
//    }
//
//    fn total_completed(&self) -> usize {
//        self.entries.iter().filter(|e| Filter::Completed.fit(e)).count()
//    }
//
//    fn is_all_completed(&self) -> bool {
//        let mut filtered_iter = self.entries
//            .iter()
//            .filter(|e| self.filter.fit(e))
//            .peekable();
//
//        if filtered_iter.peek().is_none() {
//            return false;
//        }
//
//        filtered_iter.all(|e| e.completed)
//    }
//
//    fn toggle_all(&mut self, value: bool) {
//        for entry in self.entries.iter_mut() {
//            if self.filter.fit(entry) {
//                entry.completed = value;
//            }
//        }
//    }
//
//    fn clear_completed(&mut self) {
//        let entries = self.entries.drain(..)
//            .filter(|e| Filter::Active.fit(e))
//            .collect();
//        self.entries = entries;
//    }
//
//    fn toggle(&mut self, idx: usize) {
//        let filter = self.filter.clone();
//        let mut entries = self.entries
//            .iter_mut()
//            .filter(|e| filter.fit(e))
//            .collect::<Vec<_>>();
//        let entry = entries.get_mut(idx).unwrap();
////        entry.completed = !entry.completed;
//    }
//
//    fn toggle_edit(&mut self, idx: usize) {
//        let filter = self.filter.clone();
//        let mut entries = self.entries
//            .iter_mut()
//            .filter(|e| filter.fit(e))
//            .collect::<Vec<_>>();
//        let entry = entries.get_mut(idx).unwrap();
////        entry.editing = !entry.editing;
//    }
//
//    fn complete_edit(&mut self, idx: usize, val: String) {
//        let filter = self.filter.clone();
//        let mut entries = self.entries
//            .iter_mut()
//            .filter(|e| filter.fit(e))
//            .collect::<Vec<_>>();
//        let entry = entries.get_mut(idx).unwrap();
////        entry.description = val;
////        entry.editing = !entry.editing;
//    }
//
//    fn remove(&mut self, idx: usize) {
//        let idx = {
//            let filter = self.filter.clone();
//            let entries = self.entries
//                .iter()
//                .enumerate()
//                .filter(|&(_, e)| filter.fit(e))
//                .collect::<Vec<_>>();
//            let &(idx, _) = entries.get(idx).unwrap();
//            idx
//        };
//        self.entries.remove(idx);
//    }
//}
