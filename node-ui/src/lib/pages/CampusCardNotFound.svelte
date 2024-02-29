<script lang="ts">
    import CountdownTimer from "$lib/components/CountdownTimer.svelte";

    export const stepNumber: string | number = "?";
</script>
<CountdownTimer idleTimeoutMS={5000} />
<h2>Couldn't find your campus card!<br/>Please make sure you have first registered at the front desk, or try scanning again.</h2> 