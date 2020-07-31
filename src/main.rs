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

fn fence_string(s: String) -> String {
    format!("```{}```", s)
}

#[command]
fn dcp(ctx: &mut Context, msg: &Message) -> CommandResult {
    let num = msg.content.chars().skip(4).collect::<Vec<_>>();
    let string_number = String::from_iter(num).trim().to_string();
    let number_of_questions = fs::read_dir("./questions")
        .unwrap()
        .collect::<Vec<_>>()
        .len();

    let result = string_number
        .parse::<i32>()
        .map(|n| read_input(&format!("./questions/{}", n)));
    let response = match result {
        Ok(content) => fence_string(content.unwrap()),
        Err(_) => format!(
            "Couldn't find that question. Try with a number from 1 - {}.",
            number_of_questions
        ),
    };

    msg.reply(&ctx, response);

    Ok(())
}

#[command]
fn dsp(ctx: &mut Context, msg: &Message) -> CommandResult {
    let num = msg.content.chars().skip(4).collect::<Vec<_>>();
    let string_number = String::from_iter(num).trim().to_string();
    let number_of_questions = fs::read_dir("./dsp").unwrap().collect::<Vec<_>>().len();

    let result = string_number
        .parse::<i32>()
        .map(|n| read_input(&format!("./dsp/{}", n)));
    let response = match result {
        Ok(content) => fence_string(content.unwrap()),
        Err(_) => format!(
            "Couldn't find that question. Try with a number from 1 - {}.",
            number_of_questions
        ),
    };

    msg.reply(&ctx, response);

    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(
        &ctx, format!(r#"I take five commands, starting with the prefix `!`.
If you say !dcp `$num` I will try to fetch the daily coding problem that corresponds to that day.
If you say !dsp `$num` I will try to fetch the data science prep problem that corresponds to that day.
If you say !rand, I will find you a random daily coding problem.
If you say !list, I will tell you how many questions I have (unfortunately they are not consecutive, so I might have question 1, but not 3.)"#))?;

    Ok(())
}

#[command]
fn rand(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut paths = Vec::new();

    for entry in fs::read_dir("./questions")? {
        let dir = entry?;
        paths.push(dir.path());
    }

    let mut rng = rand::thread_rng();
    let random_question = rng.gen_range(0, paths.len() - 1);

    let file_name = &paths[random_question];

    let question = read_to_string(file_name).unwrap();

    let question_number = format!("{:?}", file_name);

    let question_number = &question_number[13..question_number.len() - 1];

    msg.reply(
        &ctx,
        format!(
            "Here's question number {}: {}",
            question_number,
            fence_string(question)
        ),
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
