
<div class="app">

	<main>
		<slot />
	</main>

</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
		text-align: center;
	}

	main {
		flex: 1;
	}
</style>
