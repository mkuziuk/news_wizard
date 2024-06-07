
export type Article = {
    id: string,
    api_id: string,
    title: string,
    link: string,
    source_id: string,
    keywords: string[],
    authors: string[],
    summary: string,
    publication_date: Date,
    content: string,
    country: string[],
    category: string[],
    language: string,
    tags: string[],
    translations: Map<string, string>,
}

export type ResponseObject = {
    hits: Article[],
    quary: string,
    processingTimeMs: number,
    limit: number,
    offset: number,
    estimatedTotalHits: number
}

export const fetchArticles = async (query: string = "Crypto") => {
    let articles: Article[] = [];

    const res = await fetch(
        "http://localhost:7700/indexes/news_articles/search",
        {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                Authorization: "Bearer super_cool_key",
            },
            body: JSON.stringify({
                q: query,
            }),
        },
    );
    let responseObject: ResponseObject = await res.json();

    articles = responseObject.hits;

    console.log(articles);

    return articles;
}