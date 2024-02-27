<script lang="ts">
    import StepNumber from "./StepNumber.svelte";
    export let stepNumber: string | number = 1;
</script>
<div class="container">
    <StepNumber step={stepNumber} />
    <h2><slot/></h2>
</div>

<style lang="scss">
    .container {
        display: flex;
        justify-content: center;
        margin-bottom: 4rem;
        h2 {
            margin-left: 2rem;
            display: flex;
            justify-content: left;
            align-self: center;
            font-size: 2rem;
        }
    }
</style>