<script lang="ts">
    import { resetPage } from "$lib/api";
    import { defaultUserPage, userPage } from "$lib/stores";
    import { onDestroy, onMount } from "svelte";
    import { tweened } from "svelte/motion";
    import { writable } from "svelte/store";

    export let idleTimeoutMS = 3000;

    let startTime = Date.now();
    
    let progressBar = tweened(0);

    let interval: number | null = null;

    onMount(()=>{
        startTime = Date.now();
        interval = setInterval(()=>{
            let elapsed = Date.now() - startTime;
            let progress = elapsed / idleTimeoutMS;
            if (progress > 1.1 && interval) {
                clearInterval(interval);
                console.log("Idle timeout reached");
                resetPage();
            }
            progressBar.set(progress * 100);
        }, 100);
    });

    onDestroy(()=>{
        if (interval) {
            clearInterval(interval);
        }
    });

    function userEvent() {
        startTime = Date.now();
    }
</script>

<svelte:window on:mousemove={userEvent} on:click={userEvent} on:keyup={userEvent} />

<div class="progress-bar" style="--progress: {`${Math.min($progressBar, 100)}%`}"></div>

<style lang="scss">
    @use "../../routes/vars.scss" as *;
    .progress-bar {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 0.5em;
        background: $colorText;
        z-index: 1000;
        transition: width 0.1s;
        width: var(--progress);
    }
</style>