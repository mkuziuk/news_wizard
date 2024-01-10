import { Url } from "./url";

export class UrlService {
    baseUrl: string;
    apiKey: string;

    urlObj: Url;

    constructor(baseUrl: string, apiKey: string, urlObj?: Url) {
        this.baseUrl = baseUrl;
        this.apiKey = apiKey;
        this.urlObj = urlObj || {} as Url;
    }

    getUrl(): string {

        let url: string = this.baseUrl + "?api-key=" + this.apiKey;

        type UrlObjectKey = keyof typeof this.urlObj;

        Object.keys(this.urlObj).forEach(key => {
            const value = this.urlObj[key as UrlObjectKey];

            if (!value) return;

            switch (key) {
                case "text":
                    url += "&text=" + value;
                    break;
                case "sourceCountries":
                    url += "&source-countries=" + value;
                    break;
                case "language":
                    url += "&language=" + value;
                    break;
                case "minSentiment":
                    url += "&min-sentiment=" + value;
                    break;
                case "maxSentiment":
                    url += "&max-sentiment=" + value;
                    break;
                case "earliestPublishDate":
                    url += "&earliest-publish-date=" + value;
                    break;
                case "latestPublishDate":
                    url += "&latest-publish-date=" + value;
                    break;
                case "newsSources":
                    url += "&sources=" + value;
                    break;
                case "authors":
                    url += "&authors=" + value;
                    break;
                case "entities":
                    url += "&entities=" + value;
                    break;
                case "locationFilter":
                    url += "&location=" + value;
                    break;
                case "sort":
                    url += "&sort=" + value;
                    break;
                case "sortDirection":
                    url += "&sort-direction=" + value;
                    break;
                case "offset":
                    url += "&offset=" + value;
                    break;
                case "number":
                    url += "&number=" + value;
                    break;
            }
        });

        return url;
    }
}