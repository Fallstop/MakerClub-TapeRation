<script lang="ts">
    import { login, set_tape, add_tape, add_global_tape, add_user, reroll_name } from '$lib/api'
    import { onMount } from 'svelte';
    import { fuzzy } from 'fast-fuzzy';
    import { get_all_participants } from '$lib/api';
    import { writable } from 'svelte/store';
    import type { User } from '$lib/stores';
    
    onMount(() => {
        login()
        load_users_loop()
    })

    function load_users_loop() {
        load_users()
        setTimeout(() => {
            load_users_loop()
        }, 1000)
    }

    let search = writable('');

	function filter(user: User, search: string) {
		if (!search) {
			return true;
		}
		return fuzzy(search, user.nick_name) > 0.75;
	}


    let users: User[] = [];
    async function load_users() {
        users = (await get_all_participants()).sort((id1, id2) => id1.id - id2.id);
    }

    function handle_set_tape(user: User, form: HTMLFormElement) {
        const tape = form.tape.value - user.tape_left_cm;
        add_tape(user.campus_card, tape);
        console.log('set tape', user, tape);

        setTimeout(() => {
            load_users();
        }, 200);
    } 

    function handle_add_tape(user: User, form: HTMLFormElement) {
        const tape = form.tape.value;
        add_tape(user.campus_card, tape);
        console.log('add tape', user, tape);

        setTimeout(() => {
            load_users();
        }, 200);
    }

    function handle_add_global_tape() {
        const tapeElement = document.getElementById('global-add') as HTMLInputElement;
        const tape = tapeElement ? tapeElement.value as unknown as number || 0 : 0;
        add_global_tape(tape);
        setTimeout(() => {
            load_users();
        }, 200);
    }

    function handle_reroll_name(user: User) {
        reroll_name(user.campus_card);

        setTimeout(() => {
            load_users();
        }, 200);
    }

    function handle_add_user(form: HTMLFormElement) {
        const campus_card = form.campus_card.value;
        add_user(campus_card);
        console.log('add user', campus_card);
        form.reset();

        setTimeout(() => {
            load_users();
        }, 200);
    }

</script>

<h1>Admin Dashboard</h1>

<div id="universal-data-controls">
    <h2>Total Tape Allocated: {users.reduce((acc, user) => acc + user.tape_left_cm, 0)}cm</h2>
    <input id="global-add" type="number" value="0"/>
    <button on:click={() => {handle_add_global_tape()}}>Add to All</button>
</div>

<div class="search">
	<input type="text" placeholder="Search Users" bind:value={$search} />
</div>

<div id="users">
    <table>
        <tr>
            <th>ID</th>
            <th>User</th>
            <th></th>
            <th>Campus Card</th>
            <th>Tape (cm)</th>
            <th>Add (cm)</th>
        </tr>
        {#each users as user}
            <tr class:hidden={!filter(user, $search)}>
                <td>{user.id}</td>
                <td><p>{user.nick_name}</p></td>
                <td><button on:click={() => {handle_reroll_name(user)}}>New</button></td>
                <td>{user.campus_card}</td>
                <td>
                    <form class="user-tape-form" on:submit|preventDefault={(form) => {handle_set_tape(user, form.currentTarget)}}>
                        <input class="user-tape" name="tape" type="number" value={user.tape_left_cm}>
                        <button type="submit">Set</button>
                    </form>
                </td>
                <td>
                    <form class="user-tape-form" on:submit|preventDefault={(form) => {handle_add_tape(user, form.currentTarget)}}>
                        <input class="user-tape" name="tape" type="number" value="0">
                        <button type="submit">Add</button>
                    </form>
                </td>
            </tr>
        {/each}
    </table>
</div>

<form class="add-user" on:submit|preventDefault={(form) => {handle_add_user(form.currentTarget)}}>
    <input type="number" name="campus_card" placeholder="Campus Card" />
    <button type="submit">Add User</button>
</form>

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
        margin-bottom: 3em;
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
    td {
        .user-tape-form {
            display: flex;
            justify-content: center;
            align-items: center;
            input {
                width: 50%;
                border-radius: 10px 0 0 10px;
                border-right: none;
            }
            button {
                border-radius: 0px 10px 10px 0px;
                border: 2px solid #3f3f3f;
                border-left: none;
            }
        }
    }
    .hidden {
        display: none;
    }

    .add-user {
        display: flex;
        justify-content: center;
        align-items: center;
        input {
            width: 50%;
            border-radius: 10px 0 0 10px;
            border-right: none;
        }
        button {
            border-radius: 0px 10px 10px 0px;
            border: 2px solid #3f3f3f;
            border-left: none;
        }
        margin-bottom: 1em;
    }   
</style>
