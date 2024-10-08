import { writable } from "svelte/store"
import { SourceDemo, type Messages } from '@nekz/sdp'
export let fileLoaded = writable(false)
export let filePath = writable("")
export let gameinfoDir = writable("")
export let gameinfoLoaded = writable(false)
export let demo = writable<string>("")
export let availableDemos = writable<string[]>([])
export let pickDemo = writable<boolean>(false)
export let maxTick = writable<number>(0)
export let demoBytes = writable<Uint8Array>(new Uint8Array([]))