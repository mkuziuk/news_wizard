export type NewsItem = {
    title: string;
    text: string;
    url: string;
    image: string;
    publishDate: string;
    author: string;
    language: string;
    sourceCountry: string;
    sentiment: string;
}

/**
 * NewsItem without text.
 */
export type NewsItemCut = {
    title: string;
    url: string;
    image: string;
    publishDate: string;
    author: string;
    language: string;
    sourceCountry: string;
    sentiment: string;
}