use paris::{ log };

const EVENT_DONE: &str = "bright green";
const EVENT_CLONE: &str = "cyan";
const EVENT_ERROR: &str = "bright red";


pub struct Event<'a> {
    pub color: &'a str,
    pub text: &'a str
}

pub enum Type<'a> {
    Clone(&'a str),
    Done(&'a str),
    Error(&'a str)
}

impl Type<'_> {
    pub fn show(&self) {
        let event_info = self.get_event_info().unwrap();
        let message = self.get_message();

        let formatted_message = format!("<{}>{}</>: {}",
            event_info.color,
            event_info.text,
            message
        );

        log!("{}", formatted_message);
    }


    fn get_event_info(&self) -> Option<Event> {
        let info = match *self {
            Type::Clone(_) => Event { color: EVENT_CLONE, text: "Cloning" },
            Type::Done(_) => Event { color: EVENT_DONE, text: "Done" },
            Type::Error(_) => Event { color: EVENT_ERROR, text: "Error" },
        };

        Some(info)
    }

    fn get_message(&self) -> &str {
        match *self {
            Type::Clone(value) => value,
            Type::Done(value) => value,
            Type::Error(value) => value
        }
    }
}
