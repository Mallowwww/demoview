export type DemoMessage = {
    id: string,
    type: string,
    tick: number,
    content: Object
}
export function makeFakeMessages(length: number): DemoMessage[] {
    let messages: DemoMessage[] = []
    for (let i = 0; i < length; i++) {
        messages.push({
            id: i + " - ",
            type: '',
            tick: i,
            content: {}
        })
    }
    return messages
}
export type DemoHeader = {
    demo_protocol: number,
    net_protocol: number,
    server_name: String,
    client_name: String,
    map_name: String,
    game_dir: String,
    playback_time: number,
    tick_count: number,
    frame_count: number,
    signon_length: number
}