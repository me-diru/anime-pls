use ani_cli::search_anime;
use anyhow::Result;
use clap::Parser;
use requestty;
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    keyword: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let results = search_anime(&args.keyword)?;

    results.print_ele();
    let select_anime_question = requestty::Question::input("option")
        .message("Choose an option from the above list :D or press q to quit")
        .validate(|ans, _| {
            if (ans.parse::<usize>().unwrap() < 1) || (ans.parse::<usize>().unwrap() > 20) {
                Err(format!(
                    "Invalid option {}, Please provide another option from the list",
                    ans
                ))
            } else {
                Ok(())
            }
        })
        .build();
    let option = requestty::prompt_one(select_anime_question)?;
    match option.as_string() {
        Some(choice) => {
            if choice == "q" {
                println!("Exiting");
                return Ok(());
            }
            let anime_number: usize = choice.parse().unwrap();

            let anime = results.get_elem(anime_number - 1);
            println!("{}", anime.title);
        }
        _ => {
            println!("Invalid option");
        }
    }

    Ok(())
}
