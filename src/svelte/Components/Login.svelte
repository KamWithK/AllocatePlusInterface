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
    <h1>Login</h1>
    <div class = "group">
        <input type = "text" placeholder="Username" bind:value={username}>
    </div>

    <div class = "group">
        <input type = "password" placeholder="Password" bind:value={password}>
    </div>

    <div class = "group">
        <input type = "number" placeholder = "Authenticate Key" bind:value={authenticate}>
    </div>
    <button>Login</button>
</form>

<style>
    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 20px;
        width: fit-content;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }

    input {
        border: 0px none;
        outline: none;
        margin: 10px;
        padding: 10px;
        width: fit-content;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }

    button {
        margin: 10px;
        padding: 10px;
        font-weight: bold;
        width: 40%;
        border: 0px none;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }

    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    input[type=number] {
        -moz-appearance: textfield;
    }
</style>