<script lang="ts">
    import { fileLoaded, filePath, demo} from '$lib/filestore'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
</script>
{#if $fileLoaded == true}
    <div>{$filePath}</div>
    {#if $demo.messages}
        {#each $demo.messages as message }
            <div>
                {message.getName()}
                {message.getType()}
            </div>
        {/each}
    {/if}
{/if}