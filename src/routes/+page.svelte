<script lang="ts">
    import { fileLoaded, filePath } from '$lib/filestore'
</script>
{#if $fileLoaded == true}
    <div>{$filePath}</div>
{/if}