<script>
	let dark = false;

	function toggleDarkMode() {
		dark = !dark;
	}
</script>

<div class="container" class:dark={dark}>
	<div class="buttons">
		<button class="main-btn">Host</button>
		<button class="main-btn">Join</button>
	</div>

	<div class="dark-toggle" on:click={toggleDarkMode}>
		🌙
	</div>
</div>