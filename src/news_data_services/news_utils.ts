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

const newsUtils = Object.freeze({
    getNews,
    getNewsItemsCuts,
});

export { newsUtils };