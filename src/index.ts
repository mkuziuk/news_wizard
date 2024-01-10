import { NewsItem } from "./types/news_item";
import { newsUtils } from "./world_news_services/news_utils";
import { Url } from "./world_news_services/url/url";
import { UrlService } from "./world_news_services/url/news_api_url_service";

const apiKey: string = "33205f19e04b471683fafe23ce339c66";
const baseApiUrl: string = "https://api.worldnewsapi.com/search-news/";

let urlObj: Url = {
    text: "ukraine",
    language: "en",
    earliestPublishDate: "2024-01-02",
    number: 3,
} as Url;

const url = new UrlService(baseApiUrl, apiKey, urlObj).getUrl();


console.log(url);

async function writenews() {
    try {
        let news: NewsItem[] = await newsUtils.getNews(url);
        console.log(await newsUtils.getNewsItemsCut(news));
    }
    catch (e) {
        console.log(e);
    }
}

writenews();
