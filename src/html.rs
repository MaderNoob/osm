use crate::osm::Message;

const TEMPLATE: &str = include_str!("./template.html");

pub fn gen_html_for_message(message: &Message) -> String {
    TEMPLATE
        .replace("TITLE", &message.subject)
        .replace("SUBJECT", &message.subject)
        .replace("FROM", &message.from.to_string())
        .replace("DATE", &message.date.to_string())
        .replace("BODY", &message.body)
}
