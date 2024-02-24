<svelte:head>
    <title>Login</title>
    <meta name="description" content="Login" />
    <meta name="robots" content="noindex, nofollow" />
</svelte:head>

<script lang="ts">
    import { login } from '$lib/api'

    let password = '';

    function handleSubmit() {
        console.log('submitted', password);
        login(password);
    }

</script>


<div class="centre">
<h1>Tape Mainframe Login</h1>
<form on:submit|preventDefault="{handleSubmit}">
  <input
    class="form-field"
    bind:value="{password}"
    type="password"
    placeholder="Password"
  />
  <button class="form-field">
    Login
  </button>
</form>
</div>


<style lang="scss">
    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 50%;
    }
    input {
        font-size: large;
        padding: 0.5rem;
        border-radius: 10px;
    }
    button {
        font-size: large;
        padding: 0.5rem;
        border-radius: 10px;
        background-color: #3f3f3f;
        color: white;
        border: none;
        transition: background-color 0.3s;
        &:hover {
            background-color: #5f5f5f;
            cursor: pointer;
        }
    }
    .centre {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding-top: 10rem;
    }
</style>