<script lang="ts">
    import { fileLoaded, filePath, demo, availableDemos, pickDemo } from '$lib/filestore'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
    import { Breakdown, Viewport, Controls, Modal } from '$lib'
    let frame = 0
</script>
<div class="flex w-[100vw] h-[100vh]">
    <div class="flex flex-col w-[50%] h-[100%]">
        <div class="flex w-[100%] h-[50%]">
            <Breakdown />
        </div>
        <div class="flex w-[100%] h-[50%]">
            a
        </div>
    </div>
    <div class="flex flex-col w-[50%] h-[100%]">
        <div class="flex flex-col w-[100%] h-[50%]">
            <div class="h-[80%]">
                <Viewport ratio={"16/9"} bind:frame/>
            </div>
            <div class="h-[20%]">
                <Controls />
            </div>
            
        </div>
        <div class="flex w-[100%] h-[50%]">
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