<script lang="ts">
    import { writable } from "svelte/store";
    import { onMount } from "svelte";

    import type { Article, ResponseObject } from "./ResponseObject";
    import { fetchArticles } from "./ResponseObject";

    // If you want to save the data locally in a Svelte store for use across components
    const searchStore = writable("");

    let searchQuery = "";

    function handleSearch() {
        // You can update the store with the current search query
        searchStore.set(searchQuery);

        // If you need to save the data to a backend
        // Make an API call to your backend to save the search query
        // Replace `YOUR_API_ENDPOINT` with the actual endpoint
        console.log("Search query:", searchQuery);
    }

    export let articles: Article[];

    onMount(async () => {
        articles = await fetchArticles();
    });
</script>

<form class="max-w-lg mx-auto" on:submit|preventDefault={handleSearch}>
    <label
        for="default-search"
        class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white"
        >Search</label
    >
    <div class="relative">
        <div
            class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none"
        >
            <svg
                class="w-4 h-4 text-gray-500 dark:text-gray-400"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 20 20"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
                />
            </svg>
        </div>
        <input
            type="search"
            id="default-search"
            bind:value={searchQuery}
            on:input={() => fetchArticles(searchQuery)}
            class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            placeholder="Search for Politics, Crypto..."
            required
        />
        <button
            type="submit"
            class="text-white absolute end-2.5 bottom-2.5 bg-fuchsia-800 hover:bg-fuchsia-900 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-fuchsia-800 dark:hover:bg-fuchsia-900 dark:focus:bg-fuchsia-800"
            >Search</button
        >
    </div>
</form>
