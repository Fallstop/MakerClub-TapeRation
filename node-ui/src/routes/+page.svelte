<script lang="ts">
    import CampusCardNotFound from '$lib/pages/CampusCardNotFound.svelte';
    import RegistrationExists from '$lib/pages/RegistrationExists.svelte';
    import RegistrationSuccessful from '$lib/pages/RegistrationSuccessful.svelte';
    import TapeLengthSelection from '$lib/pages/TapeLengthSelection.svelte';
    import ScanCampusCard from '$lib/pages/ScanCampusCard.svelte';
    import { UserPage, userPage } from '$lib/stores';
    import { writable } from 'svelte/store';
    import {onMount} from 'svelte';
    import { sendMessage } from '$lib/api';

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
        console.log(event.code)
        if (event.key === "Backspace") {
            $inputValue = $inputValue.slice(0, -1);
        }
    }
</script>

<svelte:window on:keyup={handleInput} />

<h1>Finite Tape Dispenser</h1>

{#if $userPage === UserPage.ScanCampusCard}
    <ScanCampusCard />
{:else if $userPage === UserPage.CampusCardNotFound}
    <CampusCardNotFound />
{:else if $userPage === UserPage.TapeLengthSelection}
    <TapeLengthSelection />
{:else if $userPage === UserPage.RegistrationSuccessful}
    <RegistrationSuccessful />
{:else if $userPage === UserPage.RegistrationExists}
    <RegistrationExists />
{:else}
    <h2>Invalid State Exception</h2>
{/if}

<div class="hidden-input-row">
    <input class="barcode-entry" bind:value={$inputValue} disabled style="width: {$inputValue.length}ch;">
    <button on:click={()=>{$inputValue = ""}} class:hidden={!$inputValue}>ⓧ</button>
</div>


<style lang="scss">
    h1 {
        font-size: 4rem;
        text-transform: uppercase;

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
        }
    }
    .barcode-entry {
        color: #555;
    }
</style>