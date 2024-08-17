import { writable } from "svelte/store"
import { type Messages } from '@nekz/sdp'
export let fileLoaded = writable(false)
export let filePath = writable("")
export let messages = writable<Messages.Message[]>([])