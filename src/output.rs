use paris::{ log, error };


pub struct Event<'a> {
    pub color: &'a str,
    pub text: &'a str
}

pub enum Type {
    Clone,
    Done,
    Error
}

impl Type {
    pub fn text(&self) -> String {
        let event_info = self.get_event_info();

        format!("<{}>{}</>",
            event_info.color,
            event_info.text
        )
    }


    fn get_event_info(&self) -> Event {
        match *self {
            Type::Clone => Event { color: "cyan", text: "Cloning" },
            Type::Done => Event { color: "bright green", text: "Done" },
            Type::Error => Event { color: "bright red", text: "Error" },
        }
    }
}


pub struct Out {}

impl Out {
    pub fn write(event: Type, message: &str) {
        log!("{} {}", event.text(), message);
    }
}