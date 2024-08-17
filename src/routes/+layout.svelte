<script lang="ts">
    export const prerender = true
    export const ssr = false
    import { fileLoaded, filePath } from "$lib/filestore"
    import "../app.css"
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event'
    async function getfilepath() {
        invoke<string>('getfilepath').then((message) => {
            if (message != "nope") {
                $fileLoaded = true
                $filePath = message
            } else {
                $fileLoaded = false
                $filePath = ''
            }
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