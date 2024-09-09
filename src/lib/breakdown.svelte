<script lang="ts">
    import { type DemoMessage } from "$lib"
    import { ScrollArea, ScrollAreaScrollbar, Scrollbar } from '$lib/components/ui/scroll-area'
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    export let selectedMessage: string = ''
    export let messages = writable<DemoMessage[]>([
    ])
    onMount(() => {
        for (let i = 0; i < 20; i++) {
            $messages.push({
                id: i + "-",
                tick: i,
                content: {},
                type: ""
            })
            
        }
        $messages = $messages
    })
</script>
<ScrollArea type="always" class="h-[100%] w-[100%] ">
    {#each $messages as message (message.id)}
        <div class="dark:text-white pl-1 text-sm py-[.10rem] bg-foreground bg-opacity-20 odd:bg-opacity-50">{message.tick}</div>
    {/each}
    <Scrollbar orientation="vertical"/>
</ScrollArea>