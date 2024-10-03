<script lang="ts">
    import { type DemoMessage } from "$lib"
    import { ScrollArea, ScrollAreaScrollbar, Scrollbar } from '$lib/components/ui/scroll-area'
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { cn } from "./utils";
    export let selectedMessage: string = ''
    export let messages = writable<DemoMessage[]>([])
    
    onMount(() => {
        $messages = $messages
    })
</script>
<ScrollArea type="always" class="h-[100%] w-[100%] ">
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    {#each $messages as message (message.id)}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div 
            on:click={() => {selectedMessage = message.id}}
            class={
                cn("dark:text-white pl-1 text-sm py-[.10rem] bg-foreground bg-opacity-20 odd:bg-opacity-50 transition-all hover:bg-opacity-0", 
                    message.id == selectedMessage ? "bg-primary  odd:bg-opacity-80 bg-opacity-80 hover:bg-opacity-100" : ''
                )
            } 
            >{message.tick}</div>
    {/each}
    <Scrollbar orientation="vertical"/>
</ScrollArea>