export type DemoMessage = {
    id: string,
    type: string,
    tick: number,
    content: {

    }
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