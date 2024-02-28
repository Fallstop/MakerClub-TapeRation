<script lang="ts">
    import IconContainer from "$lib/components/IconContainer.svelte";
    export const stepNumber: string | number = "1";
    import ScanBarcode from "$lib/svg/ScanBarcode.svelte";

</script>
<h2>Scan the barcode on the back of your Campus Card</h2> 
<IconContainer>
    <ScanBarcode/>
</IconContainer>