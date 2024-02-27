<script>

    import PageTitle from "$lib/components/PageTitle.svelte";
    import { cardBalance } from "$lib/stores";

</script>
<PageTitle stepNumber={"2"}>Select the amount of tape you would like to withdraw</PageTitle> 
<div>{$cardBalance}</div>