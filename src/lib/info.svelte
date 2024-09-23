<script lang="ts">
    import {writable, type Writable} from 'svelte/store'
    export let strings: Writable<{
        data: string,
        type: "info" | "warn" | "err"
    }[]> = writable([
        {
            data: "This is what the info looks like",
            type: "info"
        },
        {
            data: "This is what the err looks like",
            type: "err"
        },
        {
            data: "This is what the warn looks like",
            type: "warn"
        },
        {
            data: "This is what the info looks like",
            type: "info"
        },
        {
            data: "This is what the err looks like",
            type: "err"
        },
        {
            data: "This is what the warn looks like",
            type: "warn"
        },
    ])
</script>
<div class="w-[100%] h-[100%] text-white overflow-y-scroll">
    {#each $strings as line}
        <div class="data-[type='warn']:bg-yellow-500 data-[type='err']:bg-red-500 data-[type='info']:bg-foreground data-[type]:bg-opacity-20 mt-1 mx-1 px-1 transition-all data-[type]:hover:bg-opacity-10 select-none" data-type={line.type}>
            {line.data}
        </div>
    {/each}
</div>