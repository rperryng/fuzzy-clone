use anyhow::Result;
use skim::prelude::*;
use std::io::Cursor;

pub fn fuzzy(input: Vec<String>) -> Result<Vec<String>> {
    let input = input.join("\n");
    let options = SkimOptionsBuilder::default().multi(true).build().unwrap();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new())
        .iter()
        .map(|item| item.output().to_string())
        .collect();

    Ok(selected_items)
}
