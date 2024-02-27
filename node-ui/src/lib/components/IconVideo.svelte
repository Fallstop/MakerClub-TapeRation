<script lang="ts">

    import ScanBarcode from "$lib/icons/ScanBarcode.svg?url";

</script>
<div class="container">
    <div class="icon-wrapper">
        <video loop muted autoplay style="clip-path: url('{ScanBarcode}#logo-clip')">
            <source src="/assets/gradient-random-sm.mp4" type="video/mp4">
        </video>
    </div>
</div>

<style lang="scss">
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        .icon-wrapper {
            width: 20em;
            height: 20em;
            max-width: 50vw;
            position: relative;
            video {
                
                opacity: 0.5;
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                object-fit: cover;
                z-index: -1;
            }
        }
    }
</style>