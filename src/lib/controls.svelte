<script lang="ts">
    import { Slider } from '$lib/components/ui/slider'
    import Icon from '@iconify/svelte'
    export let value: number[] = [0]
    export let maxTick: number = 0
</script>
<div class="m-auto mt-1 w-[80%] flex flex-col">
    <div class="mb-2 flex">
        <button class="ml-auto mr-1 border-2 rounded-md" style="color: hsl(var(--border))"><Icon icon="material-symbols:play-arrow" class="size-6"/></button>
        <button class="mr-auto ml-1 border-2 rounded-md " style="color: hsl(var(--border))"><Icon icon="material-symbols:stop" class="size-6"></Icon></button>
    </div>
    <Slider bind:value={value} bind:max={maxTick} step={1} />
</div>