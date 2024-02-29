<script lang="ts">
    import CountdownTimer from "$lib/components/CountdownTimer.svelte";

    export const stepNumber: string | number = "?";
</script>
<CountdownTimer idleTimeoutMS={5000} />
<h2>Registration already exists!</h2> 