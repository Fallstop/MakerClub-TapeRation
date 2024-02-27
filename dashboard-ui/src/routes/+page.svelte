<script lang="ts">
    import { login, set_tape, add_tape } from '$lib/api'
    import { onMount } from 'svelte';
    import { fuzzy } from 'fast-fuzzy';
    import { get_all_participants } from '$lib/api';
    import { writable } from 'svelte/store';
    import type { User } from '$lib/stores';
    
    onMount(() => {
        login()
        load_users()
    })

    /*
    export type User = {
        id: number;
        campus_card: string;
        nick_name: string;
        date_registered: string;
        last_transaction: string;
        tape_left_cm: number;
    };
    */

    let search = writable('');

	function filter(user: User, search: string) {
		if (!search) {
			return true;
		}
		return fuzzy(search, user.nick_name) > 0.75;
	}


    let users: User[] = [];
    function load_users() {
        get_all_participants().then((data) => {
            users = data;
        });
    }

    function handle_set_tape(user: User, form: HTMLFormElement) {
        const tape = form.tape.value - user.tape_left_cm;
        add_tape(user.id, tape);
        console.log('set tape', user, tape);
    } 

    function handle_add_tape(user: User, form: HTMLFormElement) {
        const tape = form.tape.value;
        add_tape(user.id, tape);
        console.log('add tape', user, tape);
    }

</script>

<h1>Admin Dashboard</h1>

<div id="universal-data-controls">
    <h2>Total Tape Allocated: {users.reduce((acc, user) => acc + user.tape_left_cm, 0)}cm</h2>
    <input type="number" value="0"/>
    <button>Add to All</button>
</div>

<div class="search">
	<input type="text" placeholder="Search Users" bind:value={$search} />
</div>

<div id="users">
    <table>
        <tr>
            <th>User</th>
            <th>Tape (cm)</th>
            <th>Add (cm)</th>
        </tr>
        {#each users as user}
            <tr class:hidden={!filter(user, $search)}>
                <td>{user.nick_name}</td>
                <td>
                    <form on:submit|preventDefault={(form) => {handle_set_tape(user, form.currentTarget)}}>
                        <input name="tape" type="number" value={user.tape_left_cm}>
                        <button type="submit">Set</button>
                    </form>
                </td>
                <td>
                    <form on:submit|preventDefault={(form) => {handle_add_tape(user, form.currentTarget)}}>
                        <input name="tape" type="number" value="0">
                        <button type="submit">Add</button>
                    </form>
                </td>
            </tr>
        {/each}
    </table>
</div>



<style lang="scss">
    #universal-data-controls {
        width: 100%;
        display: flex;
        justify-content: center;
        flex-direction: column;
        text-align: center;
        margin-bottom: 4em;
        input {
            margin-bottom: 4px;
        }
    }
    .search {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        margin-bottom: 2em;
    }
    #users {
        width: 100%;
        display: flex;
        flex-direction: column;
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

    table {
        width: 100%;
        table-layout: fixed;
        // border: #3f3f3f 2px solid;
        // border-radius: 10px;
    }
    th, td {
        text-align: center;
        width: 33.33%;
        font-size: large;
    } 
    .hidden {
        display: none;
    }
</style>
