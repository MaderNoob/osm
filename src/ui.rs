use std::collections::HashSet;

use anyhow::Context;
use copypasta_ext::{prelude::*, x11_bin::ClipboardContext};
use inquire::error::InquireError;

use crate::{
    html::gen_html_for_message,
    osm::{self, MailAddr},
    tmp_dir::create_msg_tmp_file,
};

pub fn run() -> anyhow::Result<()> {
    let domain = choose_domain().context("failed to choose domain")?;
    let login = choose_login().context("failed to choose login")?;
    let mail_addr = MailAddr { login, domain };
    copy_to_clipboard(mail_addr.to_string()).context("failed to copy mail address to clipboard")?;
    println!();
    println!("your maill address is:");
    println!("{}", mail_addr);
    println!();
    check_messages_loop(mail_addr)?;
    Ok(())
}

pub fn check_messages_loop(mail_addr: MailAddr) -> anyhow::Result<()> {
    const REFRESH: &str = "Refresh Messages";
    loop {
        let messages = osm::get_messages(&mail_addr).context("failed to get messages")?;
        let menu_options: Vec<String> = messages
            .iter()
            .map(|msg_info| format!("{} (from {})", msg_info.subject, msg_info.from))
            .chain(std::iter::once(REFRESH.to_string()))
            .collect();
        let selected_option = inquire::Select::new(
            "Which message would you like to read?",
            menu_options.clone(),
        )
        .prompt()
        .quit_on_cancel()?;
        if selected_option == REFRESH {
            continue;
        }
        let selected_option_index = menu_options
            .iter()
            .position(|option| option == &selected_option)
            .unwrap();
        let selected_message_info = &messages[selected_option_index];
        let message = osm::read_message(&mail_addr, selected_message_info.id)
            .context("failed to read message")?;
        let html = gen_html_for_message(&message);
        let file_path = create_msg_tmp_file(selected_message_info, html)
            .context("failed to create temporary file to store message html in")?;
        webbrowser::open(&file_path).context("failed to open web browser with message html file")?;
    }
}

fn copy_to_clipboard(s: String) -> anyhow::Result<()> {
    let mut ctx = ClipboardContext::new()
        .map_err(|err| anyhow::Error::msg(err.to_string()))
        .context("failed t create clipboard provider")?;
    ctx.set_contents(s)
        .map_err(|err| anyhow::Error::msg(err.to_string()))
        .context("failed to set clipboard content")?;
    Ok(())
}

fn choose_domain() -> anyhow::Result<String> {
    let domain_list = osm::get_domain_list().context("failed to get domain list")?;
    let domain = inquire::Select::new("Choose a domain:", domain_list)
        .prompt()
        .quit_on_cancel()?;
    Ok(domain)
}

fn choose_login() -> anyhow::Result<String> {
    fn validate_login(login: &str) -> Result<(), String> {
        if login.is_empty() {
            return Err("login can't be empty".into());
        }
        if login.contains('@') {
            return Err("login can't contain the '@' symbol".into());
        }
        Ok(())
    }

    let login = inquire::Text::new("Enter your login:")
        .with_validator(&validate_login)
        .prompt()
        .quit_on_cancel()?;

    Ok(login)
}

trait QuitOnCancel<T> {
    fn quit_on_cancel(self) -> Self;
}

impl<T> QuitOnCancel<T> for Result<T, InquireError> {
    fn quit_on_cancel(self) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(err) => match err {
                InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                    std::process::exit(0)
                }
                _ => Err(err),
            },
        }
    }
}
