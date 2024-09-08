<script lang="ts">
    import { fade } from 'svelte/transition'
    import { Button } from '$lib/components/ui/button'
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
    export let isOpen: Boolean
    export let title: string
    export let type: "confirm" | "message" | "choose"
    export let text: String = "If you're seeing this, you've messed up"
    export let options: string[] = []
    export let then: Function = (result: boolean) => {console.log(result)}
    export let thenOption = (result: string) => {console.log(result)}
</script>
{#if isOpen}
    <div transition:fade={{duration: 100}}
        class="absolute top-0 left-0 h-[100vh] w-[100vw] bg-gray-950 bg-opacity-50 flex select-none">
        <div class="bg-white border-black border-2 m-auto p-10 rounded-xl flex flex-col">
            <h1 class="text-3xl">{title}</h1>
            {#if type=="message"}
                <p class="mt-2">{text}</p>
                    <Button class="mt-2 w-[60px]" on:click={() => {isOpen = false; then()}}>Okay</Button>
            {/if}
            {#if type=="confirm"}
                <p class="mt-2">{text}</p>
                <div class="flex w-[100%] mt-2">
                    <Button class="ml-0 mr-1 w-auto" on:click={() => {isOpen = false; then(true)}}>Yes</Button>
                    <Button class="mr-0 ml-1 w-auto" on:click={() => {isOpen = false; then(false)}}>No</Button>
                </div>
            {/if}
            {#if type=="choose"}
                <p class="mt-2">{text}</p>
                <ScrollArea class="w-[500px] h-[300px]">
                    {#each options as option}
                        <div>
                            <Button class="w-full mb-1 justify-start"
                            on:click={() => {isOpen = false; thenOption(option)}}>
                                {option}
                            </Button>
                            
                        </div>
                    {/each}
                </ScrollArea>
            {/if}
        </div>
    </div>
{/if}