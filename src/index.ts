import { NewsItem } from "./types/news_item";
import { newsUtils } from "./news_data_services/news_utils";
import { Url } from "./types/url";
import { UrlService } from "./news_data_services/url/news_api_url_service";

const apiKey: string = "pub_35440fe584a4c4c1e0428c4c5454dfedf14bc";
const baseApiUrl: string = "https://newsdata.io/api/1/news";

let urlObj: Url = {
    q: "украина",
    language: "ru",
    size: 3,
} as Url;

const url = new UrlService(baseApiUrl, apiKey, urlObj).getUrl();


console.log(url);

async function logNews() {
    let news: NewsItem[] = await newsUtils.getNews(url);
    console.log(await newsUtils.getNewsItemsCuts(news));
}

logNews();
