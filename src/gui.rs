use iced::advanced::Widget;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input,
    vertical_space, Button, Checkbox, Column, Container, PickList, Row, Scrollable, Text,
    TextInput,
};
use iced::{alignment, executor, font, widget, window, Subscription};
use iced::{Alignment, Application, Command, Element, Length, Theme};
use iced_aw::style::card::CardStyles;
use iced_aw::{card, modal, Modal};
use serde::{Deserialize, Serialize};

pub(crate) mod json_handling;
mod ui_theme;

#[derive(Debug)]
enum State {
    Create,
    Edit,
    NewFile,
    DeleteList,
    None,
}

#[derive(Debug)]
enum ButtonPressed {
    Create,
    Edit,
    NewFile,
    DeleteList,
}

fn get_id(mem: &mut Mem) -> i32 {
    let tasks = json_handling::read_tasks(mem.list_name.clone()).unwrap_or(Tasks::default());
    let mut id = tasks.tasks.last().unwrap_or(&TaskEntry::default()).id;

    if !json_handling::get_path("settings.json").exists() {
        json_handling::write_settings(false, json_handling::WindowPosition { x: 100, y: 100 });
    }

    let settings = json_handling::read_settings().unwrap_or(json_handling::Settings::default());
    mem.run = settings.run;

    if (!mem.run && id == 0) {
        mem.id = 0;
        id = 0;
    } else if mem.run == true {
        id += 1;
        mem.id = id;
    }

    if tasks.tasks.is_empty() {
        id = 0;
        mem.id = id;
    }
    if !settings.run {
        json_handling::write_settings(true, json_handling::WindowPosition { x: 100, y: 100 });
    }

    return id;
}

#[derive(Debug, Serialize, Deserialize)]
struct Tasks {
    tasks: Vec<TaskEntry>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            tasks: vec![TaskEntry::default()],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TaskEntry {
    id: i32,
    name: String,
    description: String,
    tags: Vec<String>,
    completed: bool,
}

impl Default for TaskEntry {
    fn default() -> Self {
        TaskEntry {
            id: 0,
            name: String::default(),
            description: String::default(),
            tags: vec![String::default()],
            completed: false,
        }
    }
}

pub struct TaskList {
    state: State,
    button_pressed: Option<ButtonPressed>,
    mem: Mem,
}

#[derive(Debug, Clone)]
struct Mem {
    task_name: String,
    task_desc: String,
    task_entries: Vec<TaskEntry>,
    editing_task_id: Option<i32>,
    file_name: String,
    list_name: String,
    search_term: String,
    search_results: Vec<TaskEntry>,
    selected_file: Option<String>,
    task_tags: Vec<String>,
    id: i32,
    run: bool,
}
impl Default for Mem {
    fn default() -> Self {
        Self {
            task_name: String::new(),
            task_desc: String::new(),
            task_entries: vec![TaskEntry::default()],
            search_term: String::new(),
            search_results: vec![TaskEntry::default()],
            editing_task_id: None,
            file_name: String::new(),
            list_name: "tasklist".to_string(),
            selected_file: Some("tasklist".to_string()),
            task_tags: vec![String::new()],
            id: 0,
            run: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),
    ButtonEditPressed(i32),
    ButtonCreatePressed,
    ButtonDeletePressed(usize),
    TaskNameChanged(String),
    TaskNameEdited(String),
    TaskDescChanged(String),
    TaskDescEdited(String),
    TagsChanged(String),
    NewFileNameChanged(String),
    TaskSubmitted,
    TaskEdited(i32),
    CheckboxChanged(i32, bool),
    CreateNewFileButton,
    CreateNewFile,
    FileSelected(String),
    DeleteListPressed,
    DeleteList,
    SearchChanged(String),
    SearchButtonPressed,
    CloseOverlay,
}

fn modify_task(mem: &mut Mem, name: String, description: String) {
    mem.task_name = name;
    mem.task_desc = description;
}
impl Application for TaskList {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: State::None,
                button_pressed: None,
                mem: Mem::default(),
            },
            font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Tasks")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FontLoaded(_) => {
                let tasks = json_handling::read_tasks(self.mem.list_name.clone())
                    .unwrap_or_else(|_| Tasks::default());
                self.mem.task_entries = tasks.tasks;
                Command::none()
            }
            Message::ButtonCreatePressed => {
                match self.state {
                    State::None => {
                        self.button_pressed = {
                            self.state = State::Create;
                            Some(ButtonPressed::Create)
                        }
                    }
                    State::Edit | State::Create | State::NewFile | State::DeleteList => {
                        self.button_pressed = None
                    }
                }
                Command::none()
            }
            Message::ButtonEditPressed(id) => {
                match self.state {
                    State::None => {
                        self.button_pressed = {
                            self.mem.editing_task_id = Some(id);
                            if let Some(task) = self
                                .mem
                                .task_entries
                                .iter()
                                .find(|entry| entry.id == self.mem.editing_task_id.unwrap())
                            {
                                self.mem.task_name = task.name.clone();
                                self.mem.task_desc = task.description.clone();
                                self.mem.task_tags = task.tags.clone();
                            }
                            self.state = State::Edit;
                            Some(ButtonPressed::Edit)
                        }
                    }
                    State::Create | State::Edit | State::NewFile | State::DeleteList => {
                        self.button_pressed = None
                    }
                }
                Command::none()
            }
            Message::ButtonDeletePressed(index) => {
                json_handling::delete_tasks(index as i32, self.mem.list_name.clone());
                let tasks = json_handling::read_tasks(self.mem.list_name.clone()).unwrap();
                self.mem.task_entries = tasks.tasks;
                Command::none()
            }
            Message::CloseOverlay => {
                self.mem.task_name = String::new();
                self.mem.task_desc = String::new();
                self.mem.task_tags.clear();
                match (&self.state, &self.button_pressed) {
                    (State::Create, Some(ButtonPressed::Create)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                        Command::none()
                    }
                    (State::Edit, Some(ButtonPressed::Edit)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                        Command::none()
                    }
                    (State::NewFile, Some(ButtonPressed::NewFile)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                        Command::none()
                    }
                    (State::DeleteList, Some(ButtonPressed::DeleteList)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                        Command::none()
                    }
                    _ => panic!("Attempted to close overlay while in another mode.!"),
                }
            }
            Message::TaskNameChanged(input) => {
                self.mem.task_name = input;
                Command::none()
            }
            Message::TaskNameEdited(input) => {
                self.mem.task_name = input;
                Command::none()
            }
            Message::TaskDescChanged(input) => {
                self.mem.task_desc = input;
                Command::none()
            }
            Message::TaskDescEdited(input) => {
                self.mem.task_desc = input;
                Command::none()
            }
            Message::TagsChanged(input) => {
                self.mem.task_tags.clear();
                self.mem.task_tags.push(input);
                Command::none()
            }
            Message::TaskSubmitted => {
                if self.mem.id > self.mem.task_entries.len() as i32 {
                    self.mem.task_entries.resize(
                        self.mem.task_entries.len() + 1,
                        TaskEntry {
                            id: self.mem.id,
                            name: String::new(),
                            description: String::new(),
                            tags: vec![String::default()],
                            completed: false,
                        },
                    );
                }

                get_id(&mut self.mem);

                let new_entry = TaskEntry {
                    id: self.mem.id,
                    name: self.mem.task_name.clone(),
                    description: self.mem.task_desc.clone(),
                    tags: vec![self.mem.task_tags.join(", ")],
                    completed: false,
                };
                self.mem.task_entries.push(new_entry.clone());

                let entry = Tasks {
                    tasks: self.mem.task_entries.clone(),
                };

                json_handling::write_task(&entry, self.mem.list_name.clone());

                let tasks = json_handling::read_tasks(self.mem.list_name.clone()).unwrap();
                self.mem.task_entries = tasks.tasks;

                self.mem.task_name = String::new();
                self.mem.task_desc = String::new();
                self.mem.task_tags.clear();

                match (&self.state, &self.button_pressed) {
                    (State::Create, Some(ButtonPressed::Create)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                    }
                    _ => println!("Attempted to close overlay while in another mode."),
                };

                Command::none()
            }
            Message::TaskEdited(id) => {
                if let Some(task) = self
                    .mem
                    .task_entries
                    .iter_mut()
                    .find(|entry| entry.id == id)
                {
                    task.name = self.mem.task_name.clone();
                    task.description = self.mem.task_desc.clone();
                    task.tags = self.mem.task_tags.clone();

                    let updated_tasks = Tasks {
                        tasks: self.mem.task_entries.clone(),
                    };

                    json_handling::write_task(&updated_tasks, self.mem.list_name.clone());

                    self.mem.task_name = String::new();
                    self.mem.task_desc = String::new();
                    self.mem.task_tags.clear();

                    match (&self.state, &self.button_pressed) {
                        (State::Edit, Some(ButtonPressed::Edit)) => {
                            self.state = State::None;
                            self.button_pressed = None;
                        }
                        _ => println!("Attempted to close overlay while in another mode.?"),
                    };

                    Command::none()
                } else {
                    Command::none()
                }
            }
            Message::CheckboxChanged(id, checked) => {
                if let Some(task) = self
                    .mem
                    .task_entries
                    .iter_mut()
                    .find(|entry| entry.id == id)
                {
                    task.completed = checked;

                    let updated_tasks = Tasks {
                        tasks: self.mem.task_entries.clone(),
                    };

                    json_handling::write_task(&updated_tasks, self.mem.list_name.clone());
                }
                Command::none()
            }
            Message::NewFileNameChanged(input) => {
                self.mem.file_name = input;
                Command::none()
            }
            Message::CreateNewFileButton => {
                match self.state {
                    State::None => {
                        self.button_pressed = {
                            self.state = State::NewFile;
                            Some(ButtonPressed::NewFile)
                        }
                    }
                    State::Edit | State::Create | State::NewFile | State::DeleteList => {
                        self.button_pressed = None
                    }
                }
                Command::none()
            }
            Message::CreateNewFile => {
                self.mem.list_name = self.mem.file_name.clone();
                json_handling::create_new_task_file(self.mem.list_name.clone());
                self.mem.file_name = String::new();
                self.mem.selected_file = Some(self.mem.list_name.clone());

                let tasks = json_handling::read_tasks(self.mem.list_name.clone()).unwrap();
                self.mem.task_entries = tasks.tasks;

                match (&self.state, &self.button_pressed) {
                    (State::NewFile, Some(ButtonPressed::NewFile)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                    }
                    _ => println!("Attempted to close overlay while in another mode.*"),
                };
                Command::none()
            }
            Message::FileSelected(file) => {
                let test = file.clone().replace(".json", "");
                self.mem.selected_file = Some(test.clone());
                self.mem.list_name = test.clone();
                match json_handling::read_tasks(test) {
                    Ok(contents) => {
                        self.mem.task_entries = contents.tasks;
                        Command::none()
                    }
                    Err(_err) => Command::none(),
                }
            }
            Message::DeleteListPressed => {
                match self.state {
                    State::None => {
                        self.button_pressed = {
                            self.state = State::DeleteList;
                            Some(ButtonPressed::DeleteList)
                        }
                    }
                    State::Edit | State::Create | State::NewFile | State::DeleteList => {
                        self.button_pressed = None
                    }
                }
                Command::none()
            }
            Message::DeleteList => {
                json_handling::delete_task_file(self.mem.list_name.clone());
                self.mem.selected_file = Some("tasklist".to_string());
                self.mem.list_name = "tasklist".to_string();

                let tasks = json_handling::read_tasks(self.mem.list_name.clone()).unwrap();
                self.mem.task_entries = tasks.tasks;

                match (&self.state, &self.button_pressed) {
                    (State::DeleteList, Some(ButtonPressed::DeleteList)) => {
                        self.state = State::None;
                        self.button_pressed = None;
                    }
                    _ => println!("Attempted to close overlay while in another mode.*"),
                };
                Command::none()
            }
            Message::SearchChanged(input) => {
                self.mem.search_term = input.clone();

                if input.is_empty() {
                    self.mem.task_entries = json_handling::read_tasks(self.mem.list_name.clone())
                        .unwrap()
                        .tasks
                        .clone();
                }

                Command::none()
            }
            Message::SearchButtonPressed => {
                let search_term = self.mem.search_term.to_lowercase();
                self.mem.task_entries = json_handling::read_tasks(self.mem.list_name.clone())
                    .unwrap()
                    .tasks
                    .iter()
                    .filter(|entry| {
                        entry.name.to_lowercase().contains(&search_term)
                            || entry.description.to_lowercase().contains(&search_term)
                            || entry.tags.join(", ").to_lowercase().contains(&search_term)
                    })
                    .cloned()
                    .collect();

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let task_entries: Vec<Element<'_, Message>> = self
            .mem
            .task_entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let name = Text::new(format!("Name: {}", &entry.name));
                let description = Text::new(format!("Description: {}", &entry.description));
                let tags = Text::new(format!("Tags: {}", &entry.tags.join(", ")));

                let delete_button = Button::new(
                    Text::new("Delete").horizontal_alignment(alignment::Horizontal::Center),
                )
                .width(60)
                .on_press(Message::ButtonDeletePressed(index))
                .style(ui_theme::button_theme());

                let edit_button = Button::new(
                    Text::new("  Edit  ").horizontal_alignment(alignment::Horizontal::Center),
                )
                .width(60)
                .on_press(Message::ButtonEditPressed(index as i32))
                .style(ui_theme::button_theme());

                let completed_box = Checkbox::new("Completed", entry.completed, move |checked| {
                    Message::CheckboxChanged(entry.id, checked)
                })
                .style(ui_theme::checkbox_theme());

                let button_column = Column::new()
                    .align_items(Alignment::Center)
                    .push(delete_button)
                    .push(edit_button)
                    .spacing(5);

                let mut task_container = Container::new(
                    Row::new()
                        .align_items(Alignment::Center)
                        .spacing(100)
                        .push(
                            Column::new()
                                .spacing(10)
                                .push(name)
                                .push(description)
                                .push(tags)
                                .width(Length::Fill),
                        )
                        .push(
                            Column::new().push(
                                Row::new()
                                    .push(completed_box)
                                    .align_items(Alignment::Center)
                                    .push(button_column)
                                    .spacing(5),
                            ),
                        ),
                )
                .style(ui_theme::container_theme())
                .width(Length::Fill)
                .padding(5);

                task_container.into()
            })
            .collect();

        let mut task_container = Column::new().spacing(10);
        for task_entry in task_entries {
            task_container = task_container.push(task_entry);
        }

        let task_scrollbar = Scrollable::new(task_container)
            .style(ui_theme::scrollable_theme())
            .width(Length::Fill)
            .height(Length::Fill);

        let underlay = match self.state {
            State::None => {
                let files_result = json_handling::get_files();
                let pick_list = match files_result {
                    Ok(file_names) => pick_list(
                        file_names[..].to_vec(),
                        self.mem.selected_file.clone(),
                        Message::FileSelected,
                    )
                    .width(150),
                    Err(_) => {
                        PickList::new(&[][..], None, Message::FileSelected) // Empty pick_list in case of error
                    }
                };

                let button = |label, message| {
                    button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                        .style(ui_theme::button_theme())
                        .width(90)
                        .on_press(message)
                };

                let search_bar = TextInput::new("Search", self.mem.search_term.as_str())
                    .on_input(Message::SearchChanged)
                    .on_submit(Message::SearchButtonPressed);
                let search_button = button("Search", Message::SearchButtonPressed);

                container(
                    column![
                        row![
                            button("New List", Message::CreateNewFileButton),
                            pick_list.style(ui_theme::pick_list_theme()),
                            button("Delete List", Message::DeleteListPressed),
                        ]
                        .spacing(5)
                        .padding(2),
                        task_scrollbar, // Ensure the task scrollbar is added after the other elements
                        row![
                            button("New Task", Message::ButtonCreatePressed),
                            search_bar.style(ui_theme::text_input_theme()).width(150),
                            search_button,
                        ]
                        .spacing(5)
                        .padding(2),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding(10),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
            }
            _ => container(task_scrollbar.width(Length::Fill).height(Length::Fill)),
        };

        let overlay = self.button_pressed.as_ref().map(|button_pressed| {
            let head_string = match button_pressed {
                ButtonPressed::Create => "New Task",
                ButtonPressed::Edit => "Edit Task",
                ButtonPressed::NewFile => "New List",
                ButtonPressed::DeleteList => "Delete List",
            };

            let body_string = match button_pressed {
                ButtonPressed::Create => {
                    let task_name_input = text_input("Name", &self.mem.task_name)
                        .on_input(Message::TaskNameChanged)
                        .on_submit(Message::TaskSubmitted)
                        .style(ui_theme::text_input_theme());

                    let task_description_input = text_input("Description", &self.mem.task_desc)
                        .on_input(Message::TaskDescEdited)
                        .on_submit(Message::TaskSubmitted)
                        .style(ui_theme::text_input_theme());

                    let tags_input = text_input("Tags", &self.mem.task_tags.join(","))
                        .on_input(Message::TagsChanged)
                        .on_submit(Message::TaskSubmitted)
                        .style(ui_theme::text_input_theme());

                    let ok_button =
                        button(text("Ok").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::TaskSubmitted)
                            .style(ui_theme::button_theme());
                    let cancel_button =
                        button(text("Cancel").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::CloseOverlay)
                            .style(ui_theme::button_theme());

                    let inputs = column![
                        task_name_input,
                        task_description_input,
                        tags_input,
                        row![ok_button.width(60), cancel_button.width(60),].spacing(10),
                    ]
                    .spacing(10);

                    inputs
                }
                ButtonPressed::Edit => {
                    let task_name_input = text_input("Name", &self.mem.task_name)
                        .on_input(Message::TaskNameEdited)
                        .on_submit(Message::TaskEdited(self.mem.editing_task_id.unwrap()))
                        .style(ui_theme::text_input_theme());

                    let task_description_input = text_input("Description", &self.mem.task_desc)
                        .on_input(Message::TaskDescEdited)
                        .on_submit(Message::TaskEdited(self.mem.editing_task_id.unwrap()))
                        .style(ui_theme::text_input_theme());

                    let tags_input = text_input("Tags", &self.mem.task_tags.join(","))
                        .on_input(Message::TagsChanged)
                        .on_submit(Message::TaskEdited(self.mem.editing_task_id.unwrap()))
                        .style(ui_theme::text_input_theme());

                    let ok_button =
                        button(text("Ok").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::TaskEdited(self.mem.editing_task_id.unwrap()))
                            .style(ui_theme::button_theme());
                    let cancel_button =
                        button(text("Cancel").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::CloseOverlay)
                            .style(ui_theme::button_theme());

                    let inputs = column![
                        task_name_input,
                        task_description_input,
                        tags_input,
                        row![ok_button.width(60), cancel_button.width(60),].spacing(10),
                    ]
                    .spacing(10);

                    inputs
                }
                ButtonPressed::NewFile => {
                    let list_name_input = text_input("List Name", &self.mem.file_name)
                        .on_input(Message::NewFileNameChanged)
                        .on_submit(Message::CreateNewFile)
                        .style(ui_theme::text_input_theme());

                    let ok_button =
                        button(text("Ok").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::CreateNewFile)
                            .style(ui_theme::button_theme());
                    let cancel_button =
                        button(text("Cancel").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::CloseOverlay)
                            .style(ui_theme::button_theme());

                    let button_row = Row::new()
                        .push(ok_button.width(Length::from(60)))
                        .push(cancel_button.width(Length::from(60)))
                        .align_items(Alignment::Center)
                        .spacing(10);

                    let inputs = column![list_name_input, button_row,].spacing(10);

                    inputs
                }

                ButtonPressed::DeleteList => {
                    let ok_button =
                        button(text("Ok").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::DeleteList)
                            .style(ui_theme::button_theme())
                            .width(Length::Fill);

                    let cancel_button =
                        button(text("Cancel").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::CloseOverlay)
                            .style(ui_theme::button_theme())
                            .width(Length::Fill);

                    let button_row = Row::new()
                        .push(ok_button)
                        .push(cancel_button)
                        .align_items(Alignment::Center)
                        .spacing(10);

                    column![button_row].into()
                }
            };

            card(text(head_string), body_string)
                .width(Length::from(500))
                .style(CardStyles::Dark)
        });

        modal(underlay, overlay)
            .backdrop(Message::CloseOverlay)
            .on_esc(Message::CloseOverlay)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
