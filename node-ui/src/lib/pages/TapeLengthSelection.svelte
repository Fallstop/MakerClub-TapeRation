<script lang="ts">
    import { cardBalance, cardNickname } from "$lib/stores";
    import { tapeOptionsCM } from "$lib/stores";
    export const stepNumber: string | number = "2";


    function dispenseTape(tapeLengthID: number) {
        console.log(`Dispensing tape of length ${tapeOptionsCM[tapeLengthID]} cm`);
    }
</script>
<h2>Ready to dispense tape!</h2> 
<div>{$cardBalance}</div>
<div class="tape-lengths">
    {#each tapeOptionsCM as tapeLength, index}
        <button class="mono-font" on:click={()=>{dispenseTape(index)}}>
            {tapeLength.toFixed(1)} CM
        </button>
    {/each}
</div>

<style lang="scss">
    @use "../../routes/vars.scss" as *;
    .tape-lengths {
        display: flex;
        flex-direction: row;
        gap: 1rem;
        justify-content: center;
        margin: 2rem 0;
        
        button {
            padding: 1rem;
            font-size: 1.5rem;
            border: none;
            border-radius: 0.5rem;
            
            background-color: #3333;
            color: $colorText;
            outline: 1px solid $colorText;
            transition: all 0.1s ease-in-out;

            &:hover {
                background-color: #eee2;
                outline: 2px solid $colorText;
                cursor: pointer;
            }
        }
        
    }
</style>