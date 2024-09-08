<script lang="ts">
    import { Canvas, T } from '@threlte/core'
    import { Text } from '@threlte/extras'
    export let ratio = "4/3"
    export let frame = 0
    export let meshData: typeof T.Mesh = T.Mesh
    import { AspectRatio } from "$lib/components/ui/aspect-ratio/index.js";
    import { MeshBasicMaterial } from 'three';
</script>
<div class="h-[100%] bg-black mx-auto"
    style="aspect-ratio: {ratio};">
    
        <Canvas>
            <Text text="AAAA" fontSize=1/>
        </Canvas>
    
</div>
