#![allow(unused)]
use crate::{config::languages::LANGUAGES, run_commands::run_command};
use console::style;
use dialoguer::{
    console::Style,
    theme::{ColorfulTheme, Theme},
    MultiSelect,
};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

pub fn set_language() -> Result<(), String> {
    println!("Configurando linguagem do sistema...");

    let language_selected = get_user_selections();

    if language_selected.len() < 1 {
        return Err("Selecione uma linguagem, nâo apenas uma ISO".to_string());
    }

    edit_locale_gen(language_selected.clone())?;

    let output_command = run_command(&mut Command::new("locale-gen"))?;
    println!("{:#?}", output_command);

    configure_locale_conf(language_selected.clone())?;

    println!("Linguagem do sistema configurada com sucesso.");
    Ok(())
}
fn custom_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();

    // Estilo para o item ativo (selecionado)
    theme.active_item_style = Style::new().fg(console::Color::Cyan).bold();

    // Estilo para o item inativo (não selecionado)
    theme.inactive_item_style = Style::new().fg(console::Color::White);

    theme
}

fn get_user_selections() -> Vec<String> {
    let selections = MultiSelect::with_theme(&custom_theme())
        .with_prompt("Selecione uma linguagem. Caso selecione so uma ISO ocasionara em error, selecione com a tecla 'espaço' uma linguagem e uma ISO!")
        .items(&LANGUAGES)
        .interact()
        .unwrap();

    let filtered_selections: Vec<String> = selections
        .into_iter()
        .filter(|&i| !LANGUAGES[i].contains("ISO"))
        .map(|i| LANGUAGES[i].to_string())
        .collect();

    filtered_selections
}

fn edit_locale_gen(language: Vec<String>) -> Result<(), String> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {}: {}", locale_gen_path, e))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| format!("Falha ao ler linha: {}", e))?;
        if line.trim() == format!("#{}", language[0].trim()) {
            line = language[0].to_string();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_gen_path, e))?;

    for line in lines {
        writeln!(file, "{}", line);
    }

    Ok(())
}

fn configure_locale_conf(language: Vec<String>) -> Result<(), String> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!("LANG={}\n", language[0]);

    let file = File::create_new(locale_conf_path).map_err(|e| format!("sla: {}", e));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_conf_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_conf_path, e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Falha ao escrever no {}: {}", locale_conf_path, e))?;
    Ok(())
}
