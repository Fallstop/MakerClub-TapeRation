<script lang="ts">
    import CountdownTimer from "$lib/components/CountdownTimer.svelte";

    export const stepNumber: string | number = "2";
</script>
<CountdownTimer idleTimeoutMS={5000} />
<h2>Registration Successful</h2>