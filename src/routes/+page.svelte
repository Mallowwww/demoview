<script lang="ts">
    import { fileLoaded, filePath, demo} from '$lib/filestore'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
    import Breakdown from '$lib/breakdown.svelte'
</script>
{#if $fileLoaded == true}
    <Breakdown bind:messages={$demo.messages} class="w-[100%] h-[100%]"/>
{/if}