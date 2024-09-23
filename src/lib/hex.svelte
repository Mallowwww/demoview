<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";

    export let bytes = writable<Uint8Array>(new Uint8Array(100))
    let lines = writable<string[]>([])
    let strings = writable<string[]>([])
    onMount(() => {
        bytes.subscribe((value) => {
            $lines = new Array(Math.ceil((value.length) / 8))
            $strings = new Array(Math.ceil((value.length) / 8))
            for (let i = 0; i < $lines.length; i++) {
                $lines[i] = ''
                $strings[i] = ''
                for (let j = 0; j < 8; j++)
                    if (i * 8 + j < $bytes.length) {
                        $lines[i] += pad($bytes[i * 8 + j].toString(16).toUpperCase(), 2) + " "
                        $strings[i] += i * 8 + j != 0 ? String.fromCharCode($bytes[i * 8 + j]) : '.'
                    }
                    else {
                        $lines[i] += "00 "
                        $strings[i] += '.'
                    }
            }
            $lines = $lines
        })
        for (let i = 0; i < $bytes.length; i++) {
            $bytes[i] = i
        }
        $bytes = $bytes
        
    })  
    function pad(num: string, size: number) {
        while (num.length < size) num = "0" + num;
        return num;
    }
    
</script>
<div class="w-[100%] h-[100%] text-white font-mono flex">
    <div class="mx-1">
        {#each $lines as line}
            <div>{line}</div>
        {/each}
    </div>
    <div class="mx-1">
        {#each $strings as line}
            <div>{line}</div>
        {/each}
    </div>
    
</div>