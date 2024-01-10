import { NewsItem, NewsItemCut } from "../types/news_item";

const apiInputToNewsItems = async (newsApiInput: any): Promise<NewsItem[]> => {
    return Promise.resolve(newsApiInput.map((newsItem: any) => {
        return {
            articleId: newsItem.article_id,
            title: newsItem.title,
            link: newsItem.link,
            keywords: newsItem.keywords,
            creator: newsItem.creator,
            videoUrl: newsItem.video_url,
            description: newsItem.description,
            content: newsItem.content,
            pubDate: newsItem.pubDate,
            imageUrl: newsItem.image_url,
            sourceId: newsItem.source_id,
            sourcePriority: newsItem.source_priority,
            country: newsItem.country,
            category: newsItem.category,
            language: newsItem.language
        } as NewsItem
    }));
}

const newsItemsToNewsItemCuts = async (newsItems: NewsItem[]): Promise<NewsItemCut[]> => {
    return Promise.resolve(newsItems.map((newsItem: NewsItem) => {
        return {
            title: newsItem.title,
            link: newsItem.link,
            keywords: newsItem.keywords,
            creator: newsItem.creator,
            description: newsItem.description,
            content: newsItem.content,
            pubDate: newsItem.pubDate,
            sourceId: newsItem.sourceId,
            country: newsItem.country,
            category: newsItem.category,
            language: newsItem.language
        } as NewsItemCut
    }));
}

const newsMapper = {
    apiInputToNewsItems,
    newsItemsToNewsItemCuts
}

export { newsMapper };