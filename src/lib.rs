use anyhow::{Result, Context};
use select::document::Document;
use select::predicate::{Class, Attr, Predicate};

pub mod util;

pub fn print_elements(document: &Document, util_vars: &util::DependantVariables,  mut writer: impl std::io::Write) -> Result<()> {
    let mut i = 1;
    for node in  document.find(Class(util_vars.search_result_class_value).descendant(Attr("class", util_vars.anime_title_tag)))    {
        if node.text().is_empty() {
            continue;
        }

        writeln!(writer, "{}. {}", i, node.text()).with_context(|| "error while writing related titles")?;
        i+=1;
    }

    Ok(())
}
