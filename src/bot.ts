import { Telegraf } from "telegraf";
import { message } from "telegraf/filters";
import { UserData } from "./types/user_data";
import { userService } from "./user_service";
import { newsUtils } from "./news_data_services/news_utils";
import { NewsItem, NewsItemCut } from "./types/news_item";
import { UrlService } from "./news_data_services/url/news_api_url_service";
import { Url } from "./types/url";


const apiKey: string = "pub_35440fe584a4c4c1e0428c4c5454dfedf14bc";
const baseApiUrl: string = "https://newsdata.io/api/1/news";

const bot = new Telegraf("6968062089:AAFpVoEeePAhnT7ea7Gkhm8_aH1H5yiFYGc");

const menu = {
    start: "start",
    addTopic: "addtopic",
    getPortion: "getportion"
}

bot.command(menu.start, async (ctx) => {
    await ctx.reply("Write down topics you want to recieve news about in this format: topic1,topic2,topic3.");

    bot.on(message("text"), async (ctx) => {
        let userData: UserData = { topics: String(ctx.message.text).split(",") };
        userService.setUserData(ctx.message.chat.id, userData);
    })
})

bot.command(menu.addTopic, async (ctx) => {

    await ctx.reply("Enter topics you want to add in this format: topic1,topic2,topic3.");

    bot.on(message("text"), async (ctx) => {
        let userData: UserData = { topics: String(ctx.message.text).split(",") };
        userService.addUserTopics(ctx.message.chat.id, userData.topics);
    })
});

bot.command(menu.getPortion, async (ctx) => {

    userService.getUserData(ctx.message.chat.id).topics.forEach(async (topic) => {
        let urlObj: Url = {
            q: topic,
            language: "en",
            size: 3,
        } as Url;

        const url = new UrlService(baseApiUrl, apiKey, urlObj).getUrl();

        const newsItems: NewsItem[] = await newsUtils.getNews(url);

        await ctx.reply(`TOPIC: ${topic}\n` + await newsUtils.getNewsItemsString(newsItems));
    })
});


bot.launch();
