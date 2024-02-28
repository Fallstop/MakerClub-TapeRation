
<div class="container">
    <div class="icon-wrapper">
        <slot/>
    </div>
</div>

<style lang="scss">
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        .icon-wrapper {
            width: 15em;
            height: 15em;
            max-width: 50vw;
            margin: 4em;
            position: relative;
            color: #eee;
        }
    }
</style>