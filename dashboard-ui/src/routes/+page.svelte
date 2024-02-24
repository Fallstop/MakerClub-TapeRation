<script lang="ts">
    import { login } from '$lib/api'
    import { onMount } from 'svelte';
    
    onMount(() => {
        login()
    })

    let users = [
        {
            name: 'user1',
            tape: 5
        },
        {
            name: 'user2',
            tape: 2
        },
        {
            name: 'user3',
            tape: 1321983
        }
    
    ]
</script>

<h1>Admin Dashboard</h1>
<div id="universal-data-controls">
    <!-- total global tape length -->
    <!-- add to all -->
</div>
<div id="users">
    <!-- list of users -->
    <!-- add tape for user -->
    <!-- set tape for user -->
    <table>
        <tr>
            <th>User</th>
            <th>Tape</th>
            <th>Add</th>
        </tr>
        {#each users as user}
            <tr>
                <td>{user.name}</td>
                <td>
                    <input type="number" value={user.tape}>
                    <button>Set</button>
                </td>
                <td>
                    <input class="add-tape" type="number" value="0"/>
                    <button class="add-tape">Add</button>
                </td>
            </tr>
        {/each}
    </table>
</div>



<style lang="scss">
</style>
