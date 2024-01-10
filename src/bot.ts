import { Telegraf } from "telegraf";
import { message } from "telegraf/filters";

const bot = new Telegraf("6968062089:AAFpVoEeePAhnT7ea7Gkhm8_aH1H5yiFYGc");

const menu = {
    addTopic: "addtopic",
}

bot.command(menu.addTopic, async (ctx) => {
    await ctx.reply("Add Topic Here");
});


bot.launch();