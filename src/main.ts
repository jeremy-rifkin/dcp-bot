import * as Discord from "discord.js";
import { strict as assert } from "assert";
import { readFileSync, readFile, readdir } from "fs";

// Setup client
const client = new Discord.Client({
	intents: [ // fuck it, everything (almost)
		Discord.Intents.FLAGS.GUILDS,
		Discord.Intents.FLAGS.GUILD_MESSAGES,
		Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS
	]
});

const questions_folder = "questions";
const dsp_folder = "dsp";

type question_entry = {
	number: string,
	content: string
};

// question number -> question content
let questions = new Map<string, string>();
let dsp_questions = new Map<string, string>();
let question_numbers: string[] = [];
let dsp_question_numbers: string[] = [];

client.on("messageCreate", msg => {
	if(msg.content.trim() == "!chelp") {
		msg.channel.send(`I take five commands, starting with the prefix \`!\`.
If you say !dcp \`$num\` I will try to fetch the daily coding problem that corresponds to that day.
If you say !dsp \`$num\` I will try to fetch the data science prep problem that corresponds to that day.
If you say !rand, I will find you a random daily coding problem.`);
	} else if(msg.content.trim() == "!rand") {
		let n = question_numbers[Math.floor(Math.random() * question_numbers.length)];
		let question = questions.get(n);
		msg.channel.send(`Here's question number ${n}: \`\`\`${question}\`\`\``);
	} else if(msg.content.trim().startsWith("!dcp")) {
		let n = msg.content.trim().substring(4).trim();
		if(questions.has(n)) {
			let question = questions.get(n);
			msg.channel.send(`Here's question number ${n}: \`\`\`${question}\`\`\``);
		} else {
			msg.channel.send(`Couldn't find that question, try again.`);
		}
	} else if(msg.content.trim().startsWith("!dsp")) {
		let n = msg.content.trim().substring(4).trim();
		if(dsp_questions.has(n)) {
			let question = dsp_questions.get(n);
			msg.channel.send(`Here's question number ${n}: \`\`\`${question}\`\`\``);
		} else {
			msg.channel.send(`Couldn't find that question, try again.`);
		}
	}
});

client.on("ready", () => {
	console.log(`Logged in as ${client.user!.tag}`);
	readdir(questions_folder, (err, files) => {
		for(let file of files) {
			if(err) console.log(err);
			else {
				//console.log(file);
				readFile(questions_folder + "/" + file, {encoding: 'utf-8'}, (err, data) => {
					if(err) console.log(err);
					else {
						questions.set(file, data);
						question_numbers.push(file);
					}
				});
			}
		}
	});
	readdir(dsp_folder, (err, files) => {
		for(let file of files) {
			if(err) console.log(err);
			else {
				//console.log(file);
				readFile(dsp_folder + "/" + file, {encoding: 'utf-8'}, (err, data) => {
					if(err) console.log(err);
					else {
						dsp_questions.set(file, data);
						dsp_question_numbers.push(file);
					}
				});
			}
		}
	});
});

client.login(readFileSync("auth.key", { encoding: "utf-8" }));
