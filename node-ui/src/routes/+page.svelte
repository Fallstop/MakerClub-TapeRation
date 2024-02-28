<script lang="ts">
    import CampusCardNotFound from '$lib/pages/CampusCardNotFound.svelte';
    import RegistrationExists from '$lib/pages/RegistrationExists.svelte';
    import RegistrationSuccessful from '$lib/pages/RegistrationSuccessful.svelte';
    import TapeLengthSelection from '$lib/pages/TapeLengthSelection.svelte';
    import ScanCampusCard from '$lib/pages/ScanCampusCard.svelte';
    import { UserPage, userPage } from '$lib/stores';
    import { writable } from 'svelte/store';
    import { sendMessage } from '$lib/api';
    import StepNumber from '$lib/components/StepNumber.svelte';

    let inputValue = writable("");
    function handleBarcode() {
        console.log($inputValue);
        let barcode = $inputValue.trim();
        sendMessage({
            barcode_data: barcode
        })
        $inputValue = "";
    }
    function handleInput(event: KeyboardEvent) {
        if (/^[0-9]$/.test(event.key)) {
            $inputValue += event.key;
        }
        if (event.key === "Enter") {
            handleBarcode();
        }
        if (event.key === "Backspace") {
            $inputValue = $inputValue.slice(0, -1);
        }
    }

    let stepNumber: string | number;
</script>

<svelte:window on:keyup={handleInput} />
<div class="flex-row">
    <StepNumber {stepNumber} />
    <h1>Finite Tape Dispenser</h1>
</div>

{#if $userPage === UserPage.ScanCampusCard}
    <ScanCampusCard bind:stepNumber />
{:else if $userPage === UserPage.CampusCardNotFound}
    <CampusCardNotFound bind:stepNumber/>
{:else if $userPage === UserPage.TapeLengthSelection}
    <TapeLengthSelection bind:stepNumber/>
{:else if $userPage === UserPage.RegistrationSuccessful}
    <RegistrationSuccessful bind:stepNumber/>
{:else if $userPage === UserPage.RegistrationExists}
    <RegistrationExists bind:stepNumber/>
{:else}
    <h2>Invalid State Exception</h2>
{/if}

<div class="hidden-input-row">
    <input class="barcode-entry" bind:value={$inputValue} disabled style="width: {$inputValue.length}ch;">
    <button on:click={()=>{$inputValue = ""}} class:hidden={!$inputValue}>ⓧ</button>
</div>


<style lang="scss">
    @use "./vars.scss" as *;
    .flex-row {
        display: flex;
        justify-content: center;
        padding: 2rem 0;
        h1 {
            font-size: 4rem;
            text-transform: uppercase;
            margin: 0;
            margin-left: 2rem;
            
        }
        
    }
    .hidden-input-row {
        margin-top: auto;

        font-size: 2rem;
        text-align: center;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        input,button {
            background: none;
            border: none;
            outline: none;
            margin: 0;
            color: $colorText;
        }
    }
</style>