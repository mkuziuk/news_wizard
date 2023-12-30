import fetch from 'node-fetch';

async function getNews(url: string) {
    const response = await fetch(url);
    const data = await response.json();
    return data;
}

const url = " https://api.worldnewsapi.com/search-news/?" + "entities=ORG:Tesla";

getNews(url).then(data => console.log(data))