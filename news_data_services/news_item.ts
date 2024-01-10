export type NewsItem = {
    articleId: string;
    title: string;
    link: string;
    keywords: string;
    creator: string;
    videoUrl: string;
    description: string;
    content: string;
    pubDate: string;
    imageUrl: string;
    sourceId: string;
    sourcePriority: number;
    country: string[];
    category: string[];
    language: string;
}

export type NewsItemCut = {
    title: string;
    link: string;
    keywords: string;
    creator: string;
    description: string;
    content: string;
    pubDate: string;
    sourceId: string;
    country: string[];
    category: string[];
    language: string;
}