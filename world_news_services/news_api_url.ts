export class Url {
    baseUrl: string;
    apiKey: string;

    text?: string;
    sourceCountries?: string;
    language?: string;
    minSentiment?: number;
    maxSentiment?: number;
    earliestPublishDate?: string;
    latestPublishDate?: string;
    newsSources?: string;
    authors?: string;
    entities?: string;
    locationFilter?: string;
    sort?: string;
    sortDirection?: string;
    offset?: number;
    number?: number;

    constructor(baseUrl: string, apiKey: string) {
        this.baseUrl = baseUrl;
        this.apiKey = apiKey;
    }

    getUrl(self: Url): string {

        let url: string = this.baseUrl + "?api-key=" + this.apiKey;

        type UrlObjectKey = keyof typeof self;

        Object.keys(self).forEach(function (key) {
            const value = self[key as UrlObjectKey];

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