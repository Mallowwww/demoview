<script lang="ts">
    import { type Messages } from '@nekz/sdp'
    import { fileLoaded, filePath, demo} from '$lib/filestore'
    import { fade } from 'svelte/transition'
    import { Separator, Badge } from '$lib';
    import { writable } from 'svelte/store'
    import { onMount } from 'svelte';
    let classs: string = ''
    type TaggedMessage = {
        message: Messages.Message,
        tags: string[]
    }
    export {classs as class}
    export let messages: Messages.Message[] = []
    let taggedMessages = writable<TaggedMessage[]>([])
    let selectedTags = writable<string[]>([])
    
    let checker = (arr: string[], target: string[]) => {
        let answer = arr.length>0 ? target.every((v: string) => arr.includes(v)) : true
        console.log(answer)
        return answer
    };
    let allowedMessages = writable($taggedMessages.filter((message) => {if ($selectedTags.length == 0) return true;$selectedTags.forEach((tag) => {
        if (contains($selectedTags, tag)) return true;
    });return false}))
    selectedTags.subscribe(() => {$allowedMessages = $allowedMessages})
    demo.subscribe(() => {
        if (!$demo.messages || $demo.messages?.length == 0) return
        $taggedMessages = []
        $demo.messages.forEach((message) => {$taggedMessages = $taggedMessages.concat(
            {
                message: message,
                tags: [message.getName(), "tick_"+message.getTick()]
            }
        )})
        $taggedMessages = $taggedMessages
        reloadAllowed()
    })
    
    let contains = (arr: string[], target: string) => arr.indexOf(target) >= 0;
    let reloadAllowed = () => {
        $allowedMessages = $taggedMessages.filter((message) => {if ($selectedTags.length == 0) return true;return $selectedTags.every((tag) => {
            if (contains(message.tags, tag)) return true;
        })})
    }
    
</script>
<div class="{classs} flex flex-col" style="scrollbar-width: none;">
    <div class="flex flex-row h-8">
        {#each $selectedTags as tag}
            <Badge class="my-auto p-1 ml-1 text-[8pt] ">{tag}</Badge>
        {/each}
    </div>
    {#each $allowedMessages as message, i}
        <div 
            class="flex flex-row my-1">
            <div class="mx-1 size-[4pt] rounded-full bg-black my-auto"/>
            <div class="">
                {i}
            </div>
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            {#each message.tags as tag}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div on:click={() => {
                    if ($selectedTags.indexOf(tag)>=0) 
                        $selectedTags=$selectedTags.filter((e) => e != tag) 
                    else
                        $selectedTags=$selectedTags.concat(tag)
                    reloadAllowed()
                    console.log($allowedMessages)
                }}>
                    <Badge class="my-auto p-1 ml-1 text-[8pt] ">{tag}</Badge>
                </div>
                
            {/each}
        </div>
    
    {/each}
    
</div>