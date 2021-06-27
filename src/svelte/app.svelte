<script lang="ts">
	import Calendar from "./components/calendar.svelte";
	import Filter from "./components/filter.svelte";
	import Login from "./components/login.svelte"

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
	<div class="flex flex-auto">
		<div class="p-4 rounded-lg bg-indigo-500 bg-opacity-80 border-8 border-fuchsia-600 border-opacity-15">
			<Login on:handleLoginInfo={handleLoginInfo}/>
			<Filter/>
		</div>
		<div class="flex flex-col flex-grow m-4">
			<h1 class="self-center font-serif text-xl font-semibold text-blue-500 italic">Allocate Plus Plus</h1>
			<Calendar/>
		</div>
	</div>
</main>

<style global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>