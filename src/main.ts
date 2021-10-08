import * as Discord from "discord.js";
import { strict as assert } from "assert";
import { readFileSync, readFile, readdir } from "fs";

// Setup client
const client = new Discord.Client({
	intents: [
		Discord.Intents.FLAGS.GUILDS,
		Discord.Intents.FLAGS.GUILD_MESSAGES,
		Discord.Intents.FLAGS.GUILD_MESSAGE_REACTIONS
	]
});

const questions_folder = "questions";
const dsp_folder = "dsp";
const color = 0x000000;

// question number -> question content
let questions = new Map<number, string>();
let dsp_questions = new Map<number, string>();
let question_numbers: number[] = [];
let dsp_question_numbers: number[] = [];
let max_solution = 365;

function send_problem(channel: Discord.TextBasedChannels, n: number, content: string) {
	if(n <= max_solution) {
		content += `\n\n[**solution**](https://github.com/vineetjohn/daily-coding-problem/blob/master/solutions/problem_${n.toString().padStart(3, "0")}.py)`
	}
	let embed = new Discord.MessageEmbed()
				.setColor(color)
				.setTitle(`Problem ${n}`)
				.setDescription(content);
	channel.send({ embeds: [embed] })
				.catch((...args: any[]) => console.error(...args));
}

client.on("messageCreate", msg => {
	if(msg.content.trim() == "!chelp") {
		let embed = new Discord.MessageEmbed()
				.setColor(color)
				.setTitle(`Bot help`)
				.setDescription(`!rand: Get a random daily coding problem
!dcp \`n\`: Get daily coding problem \`n\`
!dsp \`n\`: Get data science prep problem \`n\`

[Source code](https://github.com/jeremy-rifkin/dcp-bot)`);
		msg.channel.send({ embeds: [embed] })
			.catch((...args: any[]) => console.error(...args));
	} else if(msg.content.trim() == "!rand") {
		let n = question_numbers[Math.floor(Math.random() * question_numbers.length)];
		assert(questions.has(n));
		let question = questions.get(n);
		assert(question != undefined);
		send_problem(msg.channel, n, question);
	} else if(msg.content.trim().startsWith("!dcp")) {
		let n = parseInt(msg.content.trim().substring(4).trim());
		if(n == NaN) {
			msg.reply("parse error");
			return;
		}
		if(!questions.has(n)) {
			msg.reply("Couldn't find that question");
			return;
		}
		let question = questions.get(n);
		assert(question != undefined);
		send_problem(msg.channel, n, question);
	} else if(msg.content.trim().startsWith("!dsp")) {
		let n = parseInt(msg.content.trim().substring(4).trim());
		if(dsp_questions.has(n)) {
			let question = dsp_questions.get(n);
			msg.channel.send(`Here's question number ${n}: \`\`\`${question}\`\`\``);
		} else {
			msg.reply(`Couldn't find that question`);
		}
	}
});

client.on("ready", () => {
	console.log(`Logged in as ${client.user!.tag}`);
	assert(client.user != null);
	client.user.setActivity("!chelp", {type: "PLAYING"});
	readdir(questions_folder, (err, files) => {
		for(let file of files) {
			if(err) console.log(err);
			else {
				//console.log(file);
				readFile(questions_folder + "/" + file, {encoding: 'utf-8'}, (err, data) => {
					if(err) console.log(err);
					else {
						let n = parseInt(file);
						assert(n != NaN);
						questions.set(n, data.trim());
						question_numbers.push(n);
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
						let n = parseInt(file);
						assert(n != NaN);
						dsp_questions.set(n, data.trim());
						dsp_question_numbers.push(n);
					}
				});
			}
		}
	});
});

client.login(readFileSync("auth.key", { encoding: "utf-8" }));
