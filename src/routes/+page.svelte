<script lang="ts">
    import { fileLoaded, filePath, messages } from '$lib/filestore'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
</script>
{#if $fileLoaded == true}
    <div>{$filePath}</div>
    {#each $messages as message }
        <div>
            {message.getName()}
        </div>
    {/each}
{/if}