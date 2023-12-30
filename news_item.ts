class newsItem {
    title: string;
    text: string;
    url: string;
    image: string;
    publishDate: string;
    author: string;
    language: string;
    sourceCountry: string;
    sentiment: string;

    constructor(title: string, text: string, url: string, image: string, publishDate: string, author: string, language: string, sourceCountry: string, sentiment: string) {
        this.title = title;
        this.text = text;
        this.url = url;
        this.image = image;
        this.publishDate = publishDate;
        this.author = author;
        this.language = language;
        this.sourceCountry = sourceCountry;
        this.sentiment = sentiment;
    }
}