<script lang="ts">
    import { fileLoaded, filePath, demo, availableDemos, pickDemo } from '$lib/filestore'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
    import { Breakdown, Viewport, Controls, Modal } from '$lib'
    let frame = 0
    let selectedMessage: string = ''
</script>
<div class="flex w-[100vw] h-[100vh] bg-background dark">
    <div class="flex flex-col w-[50%] h-[100%] border-r-2">
        <div class="flex w-[100%] h-[50%] border-b-2">
            <Breakdown bind:selectedMessage />
        </div>
        <div class="flex w-[100%] h-[50%] border-t-2">
            a
        </div>
    </div>
    <div class="flex flex-col w-[50%] h-[100%] border-l-2">
        <div class="flex flex-col w-[100%] h-[50%] border-b-2">
            <div class="h-[80%] mt-2">
                <Viewport ratio={"16/9"} bind:frame/>
            </div>
            <div class="h-[20%]">
                <Controls />
            </div>
            
        </div>
        <div class="flex w-[100%] h-[50%] border-t-2">
            a
        </div>
    </div>
</div>
<Modal bind:isOpen={$pickDemo} 
    title="Pick Demo File" 
    type="choose" 
    bind:options={$availableDemos} 
    text=""
    thenOption={(result) => {$demo = result}}
/>