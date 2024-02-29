<script lang="ts">
    import { onMount } from "svelte";

    export let idleTimeoutMS = 3000;

    let startTime = Date.now();

    onMount()=>{
        startTime = Date.now();
        let interval = setInterval(()=>{
            let elapsed = Date.now() - startTime;
            let progress = elapsed / idleTimeoutMS;
            if (progress > 1) {
                clearInterval(interval);
                console.log("Idle timeout reached");
            }
            document.querySelector(".progress-bar").style.width = `${progress * 100}%`;
        }, 100);
    }
</script>
<div class="progress-bar"></div>

<style lang="scss"></style>