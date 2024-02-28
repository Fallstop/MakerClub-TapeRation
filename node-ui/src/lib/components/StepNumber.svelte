<script lang="ts">
    export let stepNumber: number | string;
</script>
<div class="container">
    <div class="circle">
        {stepNumber}
    </div>
</div>

<style lang="scss">
    @use "../../routes/vars.scss" as *;
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
        border: $colorText solid 2px;
        box-sizing: border-box;
        border-radius: 50%;
    }
</style>