<svelte:head>
    <title>RISO 639</title>
    <meta name="description" content="ISO 639 standard data from the Rust crate RISO-639 as WASM pack" />
</svelte:head>
<script lang="ts">
    import {browser} from "$app/env";
    import { faSearch } from '@fortawesome/pro-duotone-svg-icons';
    import TableData from 'svelte-bwen/components/TableData.svelte';
    import { Input } from 'svelte-bwen/components/form';
    import riso from "$lib/riso";

    let columns = [
        {key: "alpha2", text: "Alpha 2"},
        {key: "alpha3", text: "Alpha 3"},
        {key: "text", text: "Name", },
    ];

    const filters = {
        search: '',
    };

    let filteredEntries = [];
    let entries = [];
    riso().then((wasm) => {
        if (!browser) { return; }

        entries = wasm.iso639.languages();
        filteredEntries = entries;
    });

    function filterEntries(filters, entries) {
        filteredEntries = entries.filter((entry) => {
            let matches = true;
            if (filters.search) {
                let values = [];
                columns.forEach((column) => {
                    values.push(entry[column.key].toLowerCase());
                });

                matches &= values.join(' ').includes(filters.search);
            }

            if (filters.country) {
                matches &= entry.countries.includes(filters.country);
            }

            return matches;
        });
    }

    function onSearch(event) {
        filters.search = event.target.value.toLowerCase();
        filterEntries(filters, entries);
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
