<script lang="ts">
    import SearchBar from "./SearchBar.svelte";
    import { articles } from "./SearchBar.svelte";
    import type { Article, ResponseObject } from "./ResponseObject";
</script>

<main class="flex flex-col flex-1 p-4">
    <div class=" flex-col flex-1 mt-40 mb-10">
        <SearchBar {articles} />
    </div>
    <section class="my-10">
        <!-- {#await articles}
            <div
                class="bg-gradient-to-r from-indigo-900 to-fuchsia-800/60 hover:from-fuchsia-800/60 hover:to-indigo-900 container mx-auto p-4 border-fuchsia-800/60 border-2 rounded-xl max-w-4xl my-4"
            >
                <h1>Loading data...</h1>
                <p>Wait for a moment</p>
            </div>
        {:then}
            {#each articles as article}
                <div
                    class="bg-gradient-to-r from-indigo-900 to-fuchsia-800/60 hover:from-fuchsia-800/60 hover:to-indigo-900 container mx-auto p-4 border-fuchsia-800/60 border-2 rounded-xl max-w-4xl my-4"
                >
                    <h1 class="text-xl py-2"><mark>{article.title}</mark></h1>
                    <p class="py-1">{article.authors}</p>
                    <a href={article.link} target="_blank">{article.link}</a>
                    <p class="py-1">{article.summary}</p>
                </div>
            {:else}
                <div
                    class="bg-gradient-to-r from-indigo-900 to-fuchsia-800/60 hover:from-fuchsia-800/60 hover:to-indigo-900 container mx-auto p-4 border-fuchsia-800/60 border-2 rounded-xl max-w-4xl my-4"
                >
                    <h1>No articles found</h1>
                    <p>Try another search</p>
                </div>
            {/each}
        {/await} -->
    </section>
</main>
