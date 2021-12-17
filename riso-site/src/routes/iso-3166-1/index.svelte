<svelte:head>
    <title>RISO 3166-1</title>
    <meta name="description" content="ISO 3166-1 standard data from the Rust crate RISO-3166-1 as WASM pack" />
</svelte:head>
<script lang="ts">
    import { browser } from '$app/env';
    import { faSearch } from '@fortawesome/pro-duotone-svg-icons';
    import TableData from 'svelte-bwen/components/TableData.svelte';
    import Input from 'svelte-bwen/components/form/Input.svelte';
    import CountryFlag from '$lib/components/CountryFlag.svelte';
    import riso from '$lib/riso';

    let columns = [
        {key: "alpha2", component: CountryFlag, componentProp: 'alpha2'},
        {key: "alpha2", text: "Alpha 2"},
        {key: "alpha3", text: "Alpha 3"},
        {key: "numeric", text: "Numeric"},
        {key: "name", text: "Name", },
        {key: "official_name", text: "Official Name"},
    ];

    let entries = [];
    let filteredEntries = entries;
    riso().then((wasm) => {
        if (!browser) { return; }

        entries = wasm.iso3166.countries();
        filteredEntries = entries
    });

    function onSearch(event) {
        const value = event.target.value.toLowerCase();

        filteredEntries = entries.filter((entry) => {
            let values = [];
            columns.forEach((column) => {
                values.push(entry[column.key].toLowerCase());
            });

            return values.join(' ').includes(value)
        });
    }
</script>

<style lang="css">
    .inputs {
        padding: 1.5em 1.5em 0;
    }
</style>

<div class="page-data">
    <div class="inputs">
        <Input on:input={onSearch} placeholder="Filter by any column" icon={faSearch} />
    </div>
    <TableData entries={filteredEntries} columns={columns} perPage=15 uid="alpha2" />
</div>
