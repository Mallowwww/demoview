<script lang="ts">
    import { onMount } from "svelte";
    import { writable, type Writable } from "svelte/store";
    import { ScrollArea, ScrollAreaScrollbar, Scrollbar } from '$lib/components/ui/scroll-area'

    export let bytes: Writable<Uint8Array>
    let lines = writable<string[]>([])
    let strings = writable<string[]>([])
    onMount(() => {
        bytes.subscribe((value) => {
            if (!value ) return;
            $lines = new Array(Math.ceil((value.length) / 16))
            $strings = new Array(Math.ceil((value.length) / 16))
            for (let i = 0; i < $lines.length; i++) {
                $lines[i] = ''
                $strings[i] = ''
                for (let j = 0; j < 16; j++)
                    if (i * 16 + j < value.length) {
                        $lines[i] += pad(value[i * 16 + j].toString(16).toUpperCase(), 2) + " "
                        if (value[i * 16 + j] == 0x0) $strings[i] += '. '
                        else if (value[i * 16 + j] == 0x20) $strings[i] += '  '
                        else if (value[i * 16 + j] < 33) $strings[i] += '. '
                        else if (value[i * 16 + j] == 0xAD) $strings[i] += '. '
                        else $strings[i] += String.fromCharCode(value[i * 16 + j]) + ' '
                    }
                    else {
                        $lines[i] += "00 "
                        $strings[i] += '. '
                    }
            }
            $lines = $lines
        })
        
    })  
    function pad(num: string, size: number) {
        while (num.length < size) num = "0" + num;
        return num;
    }
    
</script>
<ScrollArea class="font-bold w-[100%] h-[100%] text-white font-mono text-sm select-none" type="always">
    <div class="flex whitespace-pre">
        <div class="ml-auto mt-1">
            {#each $lines as line}
                <div class="even:bg-foreground even:bg-opacity-20 even:font-bold">{line}</div>
            {/each}
        </div>
        <div class="flex flex-col overflow-x-clip ml-2 mr-auto mt-1">
            {#each $strings as line}
                <div class="even:bg-foreground even:bg-opacity-20 even:font-bold">{line}</div>
            {/each}
        </div>
    </div>
</ScrollArea>