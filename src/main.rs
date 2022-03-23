use anime_pls::{get_episode_link, get_episodes, get_video_link, search_anime};
use anyhow::Result;
use clap::Parser;
use requestty;
use webbrowser;
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    keyword: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let results = search_anime(&args.keyword)?;

    results.print_ele();

    let select_anime_question = requestty::Question::input("select_anime")
        .message("Choose an option from the above list :D or press q to quit")
        .validate(|ans, _| {
            if ans == "q" {
                return Ok(());
            }
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

            let mut anime = results.get_elem(anime_number - 1);
            println!("{}", anime.title);
            anime.episodes = get_episodes(&anime)?;

            let select_episode_question = requestty::Question::input("select_episode")
                .message(format!(
                    "Choose an episode to watch from {} to {} or press q to quit",
                    anime.episodes[0], anime.episodes[1]
                ))
                .validate(|ans, _| {
                    if ans == "q" {
                        return Ok(());
                    }
                    if (ans.parse::<usize>().unwrap() < 1)
                        || (ans.parse::<usize>().unwrap()
                            > usize::try_from(anime.episodes.len()).unwrap())
                    {
                        Err(format!(
                            "Invalid option {}, Please choose an episode from the list",
                            ans
                        ))
                    } else {
                        Ok(())
                    }
                })
                .build();
            let chosen_episode = requestty::prompt_one(select_episode_question)?;

            match chosen_episode.as_string() {
                Some(choice) => {
                    if choice == "q" {
                        println!("Exiting");
                        return Ok(());
                    }

                    let episode_link =
                        get_episode_link(&anime, chosen_episode.as_string().unwrap().to_string())?;

                    let video_link = get_video_link(&episode_link)?;
                    println!("video link:{}", video_link);

                    let stream_option_question = requestty::Question::select("stream_option")
                        .message("Do you want to stream on browser or locally on mpv?")
                        .choices(vec![
                            "Play on browser without ads ",
                            "Play locally using mpv",
                        ])
                        .build();
                    let stream_option = requestty::prompt_one(stream_option_question)?;

                    match stream_option.try_into_list_item().unwrap().text.as_str() {
                        "Play on browser without ads " => {
                            println!("Opening {}", video_link);

                            webbrowser::open(video_link.as_str())?;
                        }
                        "Play locally using mpv" => {
                            println!("Not implemented yet");
                        }
                        _ => {
                            println!("Exiting");
                            return Ok(());
                        }
                    }
                }
                _ => {
                    println!("Invalid option");
                }
            }
        }
        _ => {
            println!("Invalid option");
        }
    }

    Ok(())
}
