<script lang="ts">
    export let step: number | string;
</script>
<div class="container">
    <div class="circle">
        {step}
    </div>
</div>

<style lang="scss">
    .container {
        display: flex;
        justify-content: center;
        align-self: center;
    }
    .circle {
        font-size: 3rem;
        font-weight: bold;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 1.5em;
        height: 1.5em;
        border: #333 solid 2px;
        box-sizing: border-box;
        border-radius: 50%;
    }
</style>