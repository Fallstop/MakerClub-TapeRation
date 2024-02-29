<script lang="ts">
    import Noise from "$lib/svg/Noise.svg?url";
    import { onMount } from "svelte";

    let video: HTMLVideoElement;
    $: {
        if (video) {
            video.playbackRate = 0.5;
        }
    }

</script>

<video
    class="video-background"
    src="/assets/gradient-random-sm.mp4"
    autoplay
    loop
    bind:this={video}
    muted></video
>
<div class="noise-filter" style="background: url('{Noise}')"></div>

<style lang="scss">
    video {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        object-fit: cover;
        z-index: -5;
        opacity: 0.5;
        filter: blur(10em)
    }
    .noise-filter {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -4;
        opacity: 0.4;
    }
    
    
</style>
