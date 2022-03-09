use crate::Menu;
use crate::config::Config;
use serde::{Serialize, Deserialize};

pub struct MenuPrinter {
    id: bool,
    link: Option<String>,
    config: FormatConfig
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormatConfig {
    title_ansi: String,
    date_format: String,
    date_ansi: String,
    label_text: Vec<String>,
    label_ansi: Vec<String>,
    description_ansi: String,
    description_width: u32,
    id_ansi: String,
    price_separator: String,
    price_separator_ansi: String,
    price_amount_ansi: String,
    price_group_ansi: String,
    link_ansi: String,
    not_found_text: String,
    not_found_ansi: String
}

impl std::default::Default for FormatConfig {
    fn default() -> Self {
        Self {
            date_format: "[%d.%m.%y]".to_string(),
            title_ansi: "1;4".to_string(),
            date_ansi: "0".to_string(),
            label_text: vec!["Vegetarian".to_string(), "Vegan".to_string(), "One Climate".to_string()],
            label_ansi: vec!["32".to_string(), "92".to_string(), "31".to_string()],
            description_ansi: "0".to_string(),
            description_width: 55,
            id_ansi: "90;3".to_string(),
            price_separator: " | ".to_string(),
            price_separator_ansi: "90".to_string(),
            price_amount_ansi: "0".to_string(),
            price_group_ansi: "0".to_string(),
            link_ansi: "0".to_string(),
            not_found_text: "No menus found!".into(),
            not_found_ansi: "90".into()
        }
    }
}

impl MenuPrinter {

    pub fn new(config: &Config, id: bool, link: &Option<String>) -> Self {
        Self {
            id,
            config: config.format.clone(),
            link: link.clone()
        }
    }

    pub fn print_menus(&self, menus: Vec<Menu>) {

        if menus.is_empty() {
            println!("{}\n", format_ansi(&self.config.not_found_text, &self.config.not_found_ansi));
        }
        else {
            for menu in menus {
                self.print_menu(menu);
            }
        }
    }

    pub fn print_menu(&self, menu: Menu) {
        let mut menu = format!("{title} {date} {label}\n{id}{description}\n{price}\n{link}",
            title = &self.format_title(&menu),
            date = &self.format_date(&menu),
            label = &self.format_label(&menu),
            id = &self.format_id(&menu),
            description = &self.format_description(&menu),
            price = &self.format_price(&menu),
            link = &self.format_link(&menu)
        );

        menu = menu.trim().into();

        println!("{}\n", menu);
    }


    fn format_title(&self, menu: &Menu) -> String {
        format_ansi(&menu.title, &self.config.title_ansi)
    }

    fn format_date(&self, menu: &Menu) -> String {
        format_ansi(&menu.date.format(&self.config.date_format).to_string(), &self.config.date_ansi)
    }

    fn format_label(&self, menu: &Menu) -> String {
        if menu.label == 0 { return "".to_string() }
        format_ansi(self.config.label_text.get((menu.label - 1) as usize).map_or_else(|| "Unknown Label", |s| s), self.config.label_ansi.get((menu.label - 1) as usize).map_or_else(|| "0", |s| s))
    }

    fn format_description(&self, menu: &Menu) -> String {
        format_ansi(textwrap::wrap(menu.description.as_str(), self.config.description_width as usize).join("\n").as_str(), &self.config.description_ansi)
    }

    fn format_id(&self, menu: &Menu) -> String {
        if self.id {
            let mut id = format_ansi(menu.id.as_str(), &self.config.id_ansi);
            id.push('\n');
            id
        } else {
            "".to_string()
        }
    }

    fn format_price(&self, menu: &Menu) -> String {
        menu.prices.iter()
            .map(|p| format!("{} {}", format_ansi(p.tag.to_uppercase().as_str(), &self.config.price_group_ansi), format_ansi(format!("{:.2}", p.price).as_str(), &self.config.price_amount_ansi)))
            .collect::<Vec<_>>().join(format_ansi(&self.config.price_separator, &self.config.price_separator_ansi).as_str())
    }

    fn format_link(&self, menu: &Menu) -> String {
        match &self.link {
            None => "".to_string(),
            Some(link) => {
                let mut link = String::from(link);
                if !link.ends_with('/') { link.push('/'); }
                link.push_str("menu/");
                link.push_str(menu.id.as_str());

                link = format_ansi(link.as_str(), &self.config.link_ansi);
                link.push('\n');

                link
            }
        }
    }
}

fn format_ansi(s: &str, format: &str) -> String {
    let mut target = String::new();
    target.push_str("\x1B[");
    target.push_str(format);
    target.push('m');
    target.push_str(s);
    target.push_str("\x1B[0m");
    target
}

