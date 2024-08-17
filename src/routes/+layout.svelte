<script lang="ts">
    export const prerender = true
    export const ssr = false
    import { fileLoaded, filePath, messages } from "$lib/filestore"
    import "../app.css"
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event'
    import * as taurifs from '@tauri-apps/api/fs'
    import fs from 'fs'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
    
    async function getfilepath() {
        await invoke<string>('getfilepath').then((message) => {
            if (message != "nope") {
                $fileLoaded = true
                $filePath = message
            } else {
                $fileLoaded = false
                $filePath = ''
            }
        })
        
        taurifs.readBinaryFile($filePath).then((value) => {
            const demo = SourceDemoParser.default()
                .setOptions({ userCmds: true })
                .parse(value.buffer)
                .findMessages((message) => {return true})
            $messages = demo
        })
        
        
            
    }
    onMount(async () => {
        listen('frontend_open', (event) => {
            getfilepath()
        })
        $fileLoaded = false
    })
    
</script>

<slot />