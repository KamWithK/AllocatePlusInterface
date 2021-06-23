<script lang="ts">
	import Calendar from "./Components/Calendar.svelte";
	import Login from "./Components/Login.svelte"

	async function handleLoginInfo(event) {
		let username = event.detail["username"];
		let password = event.detail["password"];
		let authenticate = event.detail["authenticate"];
		let response = await fetch(`/api/login?username=${username}&password=${password}&auth_key=${authenticate}`);
		let result = await response.json();
	}
</script>

<main>
	<Login on:handleLoginInfo={handleLoginInfo}/>
	<Calendar/>
</main>

<style>
	main{
		display: flex;
		flex-direction: row;
		align-items: flex-start;
		gap: 16px;
		justify-content: space-around;
		margin: 5px;
	}
</style>