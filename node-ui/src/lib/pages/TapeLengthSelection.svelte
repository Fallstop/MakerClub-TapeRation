<script lang="ts">
    import { resetPage, sendMessage } from "$lib/api";
    import CountdownTimer from "$lib/components/CountdownTimer.svelte";
    import { cardBalance, cardBalanceTweened, cardNickname } from "$lib/stores";
    import { tapeOptionsCM } from "$lib/stores";

    export const stepNumber: string | number = "2";


    function dispenseTape(tapeLengthID: number) {
        console.log(`Dispensing tape of length ${$tapeOptionsCM[tapeLengthID]} cm`);
        cardBalance.update((bal)=>(bal - $tapeOptionsCM[tapeLengthID]));
        sendMessage("DispenseTape",{
            tape_length_cm: $tapeOptionsCM[tapeLengthID],
            unix_timestamp: Date.now()
        });
    }

    function formatBalance(balanceAnimated: number, realBalance: number) {
        if (realBalance < 1) {
            return `${(balanceAnimated).toFixed(1)} CM`;
        }
        return `${(balanceAnimated / 100).toFixed(2)} M`;
    }
</script>
<CountdownTimer idleTimeoutMS={30000} />

<h2>Authenticated As <code>{$cardNickname}</code>, ready to dispense tape!</h2> 
<div class="balance-container">
    <span class="mono-font balance">{formatBalance($cardBalanceTweened, $cardBalance)}</span>
</div>
<div class="tape-lengths">
    {#each $tapeOptionsCM as tapeLength, index}
        <button class="mono-font" on:click={()=>{dispenseTape(index)}} disabled={tapeLength > $cardBalance}>
            {tapeLength.toFixed(1)} CM
        </button>
    {/each}
</div>
<div class="finished-container">
    <button on:click={resetPage} class="finished-button">Finished</button>
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
            &:disabled {
                background-color: #3333;
                color: #6666;
                outline: 1px solid #6666;
                cursor: not-allowed;
            }
        }
        
    }
    .balance-container {
        .balance {
            font-size: 16rem;
        }
    }
    .finished-button {
        padding: 1rem 2rem;
        font-size: 2rem;
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
</style>