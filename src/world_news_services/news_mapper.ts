import { NewsItem, NewsItemCut } from "../types/news_item";

const newsMapper = {
    apiInputToNewsItems,
    newsItemsToNewsItemCuts
}

async function apiInputToNewsItems(newsApiInput: any): Promise<NewsItem[]> {
    return Promise.resolve(newsApiInput.map((newsItem: any) => {
        return {
            title: newsItem.title,
            text: newsItem.text,
            url: newsItem.url,
            image: newsItem.image,
            publishDate: newsItem.publish_date,
            author: newsItem.author,
            language: newsItem.language,
            sourceCountry: newsItem.source_country,
            sentiment: newsItem.sentiment
        }
    }));
}

async function newsItemsToNewsItemCuts(newsItems: NewsItem[]): Promise<NewsItemCut[]> {
    return Promise.resolve(newsItems.map((newsItem: NewsItem) => {
        return {
            title: newsItem.title,
            url: newsItem.url,
            image: newsItem.image,
            publishDate: newsItem.publishDate,
            author: newsItem.author,
            language: newsItem.language,
            sourceCountry: newsItem.sourceCountry,
            sentiment: newsItem.sentiment
        }
    }));
}

export { newsMapper };