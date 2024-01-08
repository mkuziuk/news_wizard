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

        let url: string = this.baseUrl + "?apiKey=" + this.apiKey;

        type UrlObjectKey = keyof typeof this.urlObj;

        Object.keys(this.urlObj).forEach(key => {
            const value = this.urlObj[key as UrlObjectKey];

            if (!value) return;

            switch (key) {
                case "q":
                    url += "&q=" + value;
                    break;
                case "qlnTitle":
                    url += "&qlnTitle=" + value;
                    break;
                case "qlnMeta":
                    url += "&qlnMeta=" + value;
                    break;
                case "timeframe":
                    url += "&timeframe=" + value;
                    break;
                case "country":
                    url += "&country=" + value;
                    break;
                case "category":
                    url += "&category=" + value;
                    break;
                case "language":
                    url += "&language=" + value;
                    break;
                case "domain":
                    url += "&domain=" + value;
                    break;
                case "domainurl":
                    url += "&domainurl=" + value;
                    break;
                case "excludedomain":
                    url += "&excludedomain=" + value;
                    break;
                case "prioritydomain":
                    url += "&prioritydomain=" + value;
                    break;
                case "full_content":
                    url += "&full_content=" + value;
                    break;
                case "image":
                    url += "&image=" + value;
                    break;
                case "video":
                    url += "&video=" + value;
                    break;
                case "size":
                    url += "&size=" + value;
                    break;
                case "page":
                    url += "&page=" + value;
                    break;
            }
        });

        return url;
    }
}