use maud;
use maud::html;

use super::models::Message;

pub fn render_page(messages: Vec<Message>) -> String {
    (html! {
        head {
            title {"microservice"}
            style {"body {font-family: monospace"}
        }
        body {
            ul {
                @for message in &messages {
                    li {
                        (message.username) " (" (message.timestamp) "): " (message.message)
                    }
                }
            }
        }
    })
    .into_string()
}
