<script lang="ts">
    export const prerender = true
    export const ssr = false
    import { fileLoaded, demo, gameinfoDir, gameinfoLoaded, availableDemos, pickDemo } from "$lib/filestore"
    import "../app.css"
    import { toast } from "svelte-sonner";
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event'
    import { Toaster } from "$lib/components/ui/sonner";
    import * as taurifs from '@tauri-apps/api/fs'
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import fs from 'fs'
    import { DemoMessages, SourceDemoParser, type Messages } from '@nekz/sdp'
    
    async function getfilepath() {
        await invoke<string>('getfilepath').then(async (message) => {
            if (message != "nope") {
                $gameinfoLoaded = true
                $gameinfoDir = message.substring(0, message.lastIndexOf('\\'))
                await invoke<string>('getfilesinpath', { path: $gameinfoDir + "\\demos"}).then((message) => {
                    if (message == "nope") {
                        toast.error("No demos folder")
                    } else {
                        toast.success("Demo folder located")
                        let demos: string[] = []
                        message.split(",").forEach((a) => {
                            let split = a.split("\\")
                            let s = split.at(-1)
                            if (s)
                                demos.push(s)
                        })
                        $availableDemos = demos
                        $pickDemo = true
                    }
                })
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
<div class="static">
    <Toaster />
</div>