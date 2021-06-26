<script lang="ts">
	import Calendar from "./components/calendar.svelte";
	import Login from "./components/login.svelte";

	async function handleLoginInfo(event) {
		let username = event.detail["username"];
		let password = event.detail["password"];
		let authenticate = event.detail["authenticate"];

		let units_response = await fetch("/api/login", {
			method: "POST",
			body: JSON.stringify({"username": username, "password": password, "auth_key": authenticate.toString()})
		});
		let units_result = await units_response.json();

		console.log(units_result);

		let collisions_response = await fetch(`/api/collisions?units=${JSON.stringify(units_result)}`);
		let collisions_result = await collisions_response.json();

		console.log(collisions_result);
	}
</script>

<main>
	<Login on:handleLoginInfo={handleLoginInfo}/>
	<Calendar/>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	main{
		display: flex;
		flex-direction: row;
		align-items: flex-start;
		gap: 16px;
		justify-content: space-around;
		margin: 5px;
	}
</style>