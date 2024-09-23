<script lang="ts">
    import { fileLoaded, filePath, demo, availableDemos, pickDemo, gameinfoDir, maxTick, demoBytes} from '$lib/filestore'
    import { Breakdown, Viewport, Controls, Modal, type DemoMessage, makeFakeMessages, Hex } from '$lib'
    import { writable } from 'svelte/store';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri'
    let messages = writable<DemoMessage[]>([])
    let frame = writable<number[]>([0])
    let selectedMessage: string = ''
    let devText: string = 'No demo loaded'
    let bytes = writable<Uint8Array>(new Uint8Array([]))
    demo.subscribe((value) => {
        if ($demo.length > 0) {
            devText = value
            $messages = makeFakeMessages(30)
            invoke('getbytesfromfile', { path: $filePath }).then((value) => {
                $demoBytes = new Uint8Array(value as [])
            })
            
        }
        
    })
    demoBytes.subscribe((value) => {
        $bytes = value
    })
    onMount(() => {

    })
</script>
<div class="flex w-[100vw] h-[100vh] bg-background dark">
    <div class="flex flex-col w-[50%] h-[100%] border-r-2">
        <div class="flex w-[100%] h-[50%] border-b-2">
            <Breakdown bind:selectedMessage bind:messages />
        </div>
        <div class="flex w-[100%] h-[50%] border-t-2">
            a
        </div>
    </div>
    <div class="flex flex-col w-[50%] h-[100%] border-l-2">
        <div class="flex flex-col w-[100%] h-[50%] border-b-2">
            <div class="h-[80%] mt-2">
                <Viewport ratio={"16/9"} bind:frame={$frame[0]} bind:devText/>
            </div>
            <div class="h-[20%]">
                <Controls bind:value={$frame} bind:maxTick={$maxTick}/>
            </div>
            
        </div>
        <div class="flex w-[100%] h-[50%] border-t-2">
            <Hex bind:bytes/>
        </div>
    </div>
</div>
<Modal bind:isOpen={$pickDemo} 
    title="Pick Demo File" 
    type="choose" 
    bind:options={$availableDemos} 
    text=""
    thenOption={(result) => {$filePath = $gameinfoDir + "\\demos\\" + result;$demo = result; $fileLoaded = true}}
/>
