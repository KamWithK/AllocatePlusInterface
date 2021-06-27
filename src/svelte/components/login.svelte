<script>
    import {createEventDispatcher} from "svelte";

    let dispatch = createEventDispatcher();
    
    let username;
    let password;
    let authenticate;

    // Handle when user clicks submit
    const handleSubmit = () => {
        const loginInfo = {
            username: username,
            password: password,
            authenticate: authenticate
        }

        dispatch("handleLoginInfo", loginInfo);
    }
</script>

<!-- Handles login input from user -->
<form on:submit|preventDefault={handleSubmit}>
    <div class="flex flex-col gap-1 items-center">
        <h1 class="font-sans mb-2">Login</h1>

        <input type="text" placeholder="Username" bind:value={username}>
        <input type="password" placeholder="Password" bind:value={password}>
        <input type="number" placeholder="Authenticate Key" bind:value={authenticate} class="mt-2">

        <button class="w-1/2 rounded-full bg-purple-700 text-white border-4 hover:border-fuchsia-600">Login</button>
    </div>
</form>

<style global>    
    input {
        @apply border-blue-500;
    }

    input[type=number] {
        -moz-appearance: textfield;
    }
</style>