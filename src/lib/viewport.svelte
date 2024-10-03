<script lang="ts">
    import { Canvas, T } from '@threlte/core'
    import { Text } from '@threlte/extras'
    export let ratio = "4/3"
    export let frame = 0
    export let meshData: typeof T.Mesh = T.Mesh
    export let devText: string = 'No demo loaded'
    import { AspectRatio } from "$lib/components/ui/aspect-ratio/index.js";
    import { MeshBasicMaterial } from 'three';
</script>
<div class="h-[100%] bg-black e border-2 mx-auto"
    style="aspect-ratio: {ratio};">
    
        <Canvas>
            <Text bind:text={devText} fontSize=1 anchorX={4.1} anchorY={-.5}/>
        </Canvas>
    
</div>
