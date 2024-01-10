import { NewsItem, NewsItemCut } from "../types/news_item";
import { newsMapper } from "./news_mapper";

const newsUtils = {
    getNews,
    getNewsTexts,
    getNewsItemsCut,
    getNewsItemsUrls
}

async function getNews(url: string): Promise<NewsItem[]> {
    const response = await fetch(url);
    const data = await response.json();

    if (!data.news) {
        throw new Error(`NewsItems from World News API not received. \n ${data}`);
    }

    return await newsMapper.apiInputToNewsItems(data.news);
}

async function getNewsTexts(news: NewsItem[]): Promise<string[]> {
    return Promise.resolve(news.map((newsItem: NewsItem) => newsItem.text));
}

async function getNewsItemsCut(news: NewsItem[]): Promise<NewsItemCut[]> {
    return Promise.resolve(await newsMapper.newsItemsToNewsItemCuts(news));
}

async function getNewsItemsUrls(news: NewsItemCut[]): Promise<string[]> {
    return Promise.resolve(news.map((newsItem: NewsItemCut) => newsItem.url));
}

export { newsUtils };