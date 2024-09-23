<script lang="ts">
    export let strings: {
        data: string,
        type: "info" | "warn" | "err"
    }[] = [
        {
            data: "This is what the data looks like",
            type: "info"
        },
        {
            data: "This is what the data looks like",
            type: "info"
        },
        {
            data: "This is what the data looks like",
            type: "info"
        }
    ]
</script>
<div class="w-[100%] h-[100%] text-white">
    {#each strings as line}
        <div class="bg-background even:bg-secondary">
            {line.data}
        </div>
    {/each}
</div>