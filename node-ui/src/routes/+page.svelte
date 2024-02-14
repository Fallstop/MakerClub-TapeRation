<script lang="ts">
    import CampusCardNotFound from '$lib/pages/CampusCardNotFound.svelte';
    import RegistrationExists from '$lib/pages/RegistrationExists.svelte';
    import RegistrationSuccessful from '$lib/pages/RegistrationSuccessful.svelte';
    import TapeLengthSelection from '$lib/pages/TapeLengthSelection.svelte';
    import ScanCampusCard from '$lib/pages/ScanCampusCard.svelte';
    import { UserPage, userPage } from '$lib/stores';
</script>

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
    <h1>Invalid State Exception</h1>
{/if}

<h1>Tape Dispenser</h1>