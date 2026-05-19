mod modules;

use std::collections::{HashMap, HashSet};

use iced::widget::{button, checkbox, column, container, row, scrollable, text, Space};
use iced::{Element, Length, Task, Theme};

use modules::{
    list_available_modules, list_loaded_modules, read_blacklist, write_blacklist, LoadedInfo,
    ModuleEntry,
};

const BLACKLIST_PATH: &str = "/etc/modprobe.d/kmod-blacklist.conf";

#[derive(Debug, Clone)]
enum Message {
    ToggleAvailable(String, bool),
    ToggleBlocked(String, bool),
    BlockSelected,
    UnblockSelected,
    Save,
    Refresh,
}

#[derive(Default)]
struct App {
    available: Vec<ModuleEntry>,
    blocked: Vec<ModuleEntry>,
    selected_available: HashSet<String>,
    selected_blocked: HashSet<String>,
    loaded: HashMap<String, LoadedInfo>,
    status: String,
}

impl App {
    fn new() -> Self {
        let mut app = App::default();
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        let loaded = list_loaded_modules().unwrap_or_else(|e| {
            self.status = format!("Could not read /proc/modules: {e}");
            HashMap::new()
        });
        let available_names = list_available_modules().unwrap_or_else(|e| {
            self.status = format!("Could not enumerate /lib/modules: {e}");
            Vec::new()
        });

        let blocked_names = read_blacklist(BLACKLIST_PATH).unwrap_or_default();
        let blocked_set: HashSet<&String> = blocked_names.iter().collect();

        let mut available: Vec<ModuleEntry> = available_names
            .into_iter()
            .filter(|n| !blocked_set.contains(n))
            .map(|name| {
                let info = loaded.get(&name);
                ModuleEntry {
                    loaded: info.is_some(),
                    in_use: info.map(LoadedInfo::in_use).unwrap_or(false),
                    name,
                }
            })
            .collect();
        available.sort_by(|a, b| a.name.cmp(&b.name));

        let blocked: Vec<ModuleEntry> = blocked_names
            .into_iter()
            .map(|name| {
                let info = loaded.get(&name);
                ModuleEntry {
                    loaded: info.is_some(),
                    in_use: info.map(LoadedInfo::in_use).unwrap_or(false),
                    name,
                }
            })
            .collect();

        self.selected_available = available
            .iter()
            .filter(|m| !m.loaded)
            .map(|m| m.name.clone())
            .collect();
        self.selected_blocked.clear();
        self.available = available;
        self.blocked = blocked;
        self.loaded = loaded;
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleAvailable(name, checked) => {
                if checked {
                    self.selected_available.insert(name);
                } else {
                    self.selected_available.remove(&name);
                }
            }
            Message::ToggleBlocked(name, checked) => {
                if checked {
                    self.selected_blocked.insert(name);
                } else {
                    self.selected_blocked.remove(&name);
                }
            }
            Message::BlockSelected => {
                let selected: Vec<String> = self.selected_available.iter().cloned().collect();
                let mut moved = 0;
                let mut refused = Vec::new();

                let mut to_keep = Vec::new();
                for entry in self.available.drain(..) {
                    if selected.contains(&entry.name) {
                        if entry.in_use {
                            refused.push(entry.name.clone());
                            to_keep.push(entry);
                        } else {
                            if !self.blocked.iter().any(|b| b.name == entry.name) {
                                self.blocked.push(entry);
                                moved += 1;
                            }
                        }
                    } else {
                        to_keep.push(entry);
                    }
                }
                self.available = to_keep;
                self.blocked.sort_by(|a, b| a.name.cmp(&b.name));
                self.selected_available.clear();

                self.status = match (moved, refused.is_empty()) {
                    (n, true) => format!("Moved {n} module(s) to blocklist"),
                    (0, false) => format!("Refused (in use): {}", refused.join(", ")),
                    (n, false) => {
                        format!("Moved {n}; refused (in use): {}", refused.join(", "))
                    }
                };
            }
            Message::UnblockSelected => {
                let selected: Vec<String> = self.selected_blocked.iter().cloned().collect();
                let mut moved = 0;

                let mut to_keep = Vec::new();
                for entry in self.blocked.drain(..) {
                    if selected.contains(&entry.name) {
                        self.available.push(entry);
                        moved += 1;
                    } else {
                        to_keep.push(entry);
                    }
                }
                self.blocked = to_keep;
                self.available.sort_by(|a, b| a.name.cmp(&b.name));
                self.selected_blocked.clear();
                self.status = format!("Moved {moved} module(s) back to available");
            }
            Message::Save => {
                let names: Vec<String> =
                    self.blocked.iter().map(|m| m.name.clone()).collect();
                match write_blacklist(BLACKLIST_PATH, &names) {
                    Ok(()) => {
                        self.status =
                            format!("Saved {} entries to {BLACKLIST_PATH}", names.len());
                    }
                    Err(e) => {
                        self.status = format!("Save failed ({BLACKLIST_PATH}): {e}");
                    }
                }
            }
            Message::Refresh => {
                self.refresh();
                if self.status.is_empty() {
                    self.status = "Refreshed".to_string();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let available_items = self.available.iter().map(|m| {
            let info = self.loaded.get(&m.name);
            let label = if m.in_use {
                let refcount = info.map(|i| i.refcount).unwrap_or(0);
                let users = info.map(|i| i.users.len()).unwrap_or(0);
                format!("{} — loaded, in use (refcount {refcount}, {users} users)", m.name)
            } else if m.loaded {
                format!("{} — loaded", m.name)
            } else {
                m.name.clone()
            };
            let name = m.name.clone();
            let is_locked = m.in_use;
            let cb = checkbox(label, self.selected_available.contains(&m.name));
            let cb = if is_locked {
                cb
            } else {
                cb.on_toggle(move |b| Message::ToggleAvailable(name.clone(), b))
            };
            Element::from(cb)
        });
        let available_col = column(available_items).spacing(2).padding(4);

        let blocked_items = self.blocked.iter().map(|m| {
            let name = m.name.clone();
            let label = if m.loaded {
                format!("{} — currently loaded (blacklist applies after reboot)", m.name)
            } else {
                m.name.clone()
            };
            let cb = checkbox(label, self.selected_blocked.contains(&m.name))
                .on_toggle(move |b| Message::ToggleBlocked(name.clone(), b));
            Element::from(cb)
        });
        let blocked_col = column(blocked_items).spacing(2).padding(4);

        let lists = row![
            column![
                text(format!("Available ({})", self.available.len())).size(18),
                container(scrollable(available_col)).height(Length::Fill),
            ]
            .width(Length::FillPortion(1))
            .spacing(8),
            column![
                Space::with_height(Length::Fixed(28.0)),
                button("Block selected →").on_press(Message::BlockSelected),
                button("← Unblock selected").on_press(Message::UnblockSelected),
            ]
            .spacing(8)
            .padding(8),
            column![
                text(format!("Blocked ({})", self.blocked.len())).size(18),
                container(scrollable(blocked_col)).height(Length::Fill),
            ]
            .width(Length::FillPortion(1))
            .spacing(8),
        ]
        .spacing(12)
        .height(Length::Fill);

        let footer = row![
            button("Refresh").on_press(Message::Refresh),
            Space::with_width(Length::Fill),
            text(&self.status),
            Space::with_width(Length::Fill),
            button("Save").on_press(Message::Save),
        ]
        .spacing(8)
        .padding(8);

        column![lists, footer].padding(12).spacing(8).into()
    }
}

pub fn main() -> iced::Result {
    iced::application("Kernel Module Blacklist", App::update, App::view)
        .theme(|_| Theme::Dark)
        .window_size((900.0, 600.0))
        .run_with(|| (App::new(), Task::none()))
}
