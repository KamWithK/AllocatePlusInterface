<script lang="ts">
	async function submit(event) {
		let username = event.target.username.value;
		let password = event.target.password.value;
		let auth_key = event.target.auth_key.value;

		let response = await fetch(`/api/login?username=${username}&password=${password}&auth_key=${auth_key}`);
		let result = await response.json();

		console.log(result);
    }
</script>

<main>
	<h1>Allocate Plus Plus</h1>
	<form on:submit|preventDefault={submit}>
		<label for="username">Username:</label><br>
		<input type="text" autocomplete="username" id="username" name="username"><br>
		<label for="password">Password:</label><br>
		<input type="password" autocomplete="password" id="password" name="password">
		<label for="auth_key">Auth Key:</label><br>
		<input type="text" inputmode="numeric" pattern="[0-9]*" id="auth_key" name="auth_key">
		
		<input type="submit" value="Submit">
	</form>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>