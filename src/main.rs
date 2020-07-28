use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

use rand::Rng;
use std::fs;
use std::fs::read_to_string;
use std::iter::FromIterator;

#[group]
#[commands(dcp, list, rand, help)]
struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn read_input(file_name: &str) -> Result<String, std::io::Error> {
    read_to_string(file_name)
}

#[command]
fn dcp(ctx: &mut Context, msg: &Message) -> CommandResult {
    let num = msg.content.chars().skip(4).collect::<Vec<_>>();
    let string_number: String = String::from_iter(num).trim().to_string();

    let result = string_number
        .parse::<i32>()
        .map(|n| read_input(&format!("./questions/{}", n)));
    let response = match result {
        Ok(content) => content.unwrap(),
        Err(_) => "Couldn't find that question. Try with a number from 1 - 374.".to_string(),
    };

    msg.reply(&ctx, response);

    // let problem_number = string_number.parse::<i32>();

    // match problem_number {
    //     Ok(num) => {
    //         let output = read_input(&format!("./questions/{}", num));
    //         if output.is_ok() {
    //             msg.reply(&ctx, format!("Here's question number {}", num))?;
    //             msg.reply(&ctx, format!("{}", output.unwrap()))?;
    //         } else {
    //             msg.reply(&ctx, format!("Question number {} was not found.", num))?;
    //         }
    //     }
    //     Err(_) => {
    //         msg.reply(
    //             &ctx,
    //             format!(
    //                 "You wanted question number `{}`, but it couldn't be parsed.",
    //                 string_number
    //             ),
    //         )?;
    //     }
    // }

    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, format!("I take four commands, starting with the prefix `!`. If you say !dcp `$num` I will try to fetch the daily coding problem that corresponds to that day.If you say !rand, I will find you a random daily coding problem. If you say !list, I will tell you how many questions I have (unfortunately they are not consecutive, so I might have question 1, but not 3.)"))?;

    Ok(())
}

#[command]
fn rand(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, format!("Let me find you a random question."))?;

    let mut paths = Vec::new();

    for entry in fs::read_dir("./questions")? {
        let dir = entry?;
        paths.push(dir.path());
    }

    let mut rng = rand::thread_rng();
    let random_question = rng.gen_range(0, paths.len() - 1);

    let file_name = &paths[random_question];

    let question = read_to_string(file_name).unwrap();

    msg.reply(
        &ctx,
        format!("Heres question number {:?}: {}", file_name, question),
    )?;

    Ok(())
}

#[command]
fn list(ctx: &mut Context, msg: &Message) -> CommandResult {
    let questions = fs::read_dir("./questions").unwrap().collect::<Vec<_>>();
    let questions = questions.len();
    msg.reply(&ctx, format!("I have {} questions.", questions))?;

    Ok(())
}
