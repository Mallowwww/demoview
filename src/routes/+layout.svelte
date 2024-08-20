<script lang="ts">
    export const prerender = true
    export const ssr = false
    import { fileLoaded, filePath, demo, gameinfoDir, gameinfoLoaded } from "$lib/filestore"
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
                $gameinfoLoaded = true
                $gameinfoDir = message.substring(0, message.lastIndexOf('/'))
                console.log($gameinfoDir)
            } else {
                $gameinfoLoaded = false
                $gameinfoDir = ''
            }
        })  
    }
    onMount(async () => {
        listen('frontend_open', getfilepath)
        $fileLoaded = false
        $gameinfoLoaded = false
    })
    
</script>

<slot />