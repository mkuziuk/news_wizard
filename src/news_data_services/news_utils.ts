import { NewsItem, NewsItemCut } from "../types/news_item";
import { newsMapper } from "./news_mapper";

const getNews = async (url: string): Promise<NewsItem[]> => {
    const response = await fetch(url);
    const data = await response.json();
    return newsMapper.apiInputToNewsItems(data.results);
}

const getNewsItemsCuts = async (news: NewsItem[]): Promise<NewsItemCut[]> => {
    return newsMapper.newsItemsToNewsItemCuts(news);
}

const getNewsItemsString = async (news: NewsItem[]): Promise<string> => {
    let newsString: string[] = news.map((newsItem: NewsItem) => {
        return `
Title: ${newsItem.title}
Link: ${newsItem.link}
Author: ${newsItem.creator}
Description: ${newsItem.description}
PubDate: ${newsItem.pubDate}
Country: ${newsItem.country}
Category: ${newsItem.category}
        `
    });

    return newsString.join("\n");
}

export const newsUtils = Object.freeze({
    getNews,
    getNewsItemsCuts,
    getNewsItemsString
});
