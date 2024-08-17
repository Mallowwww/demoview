import { writable } from "svelte/store"

export let fileLoaded = writable(false)
export let filePath = writable("")