<svelte:head>
    <title>RISO 3166-2</title>
    <meta name="description" content="ISO 3166-2 standard data from the Rust crate RISO-3166-2 as WASM pack" />
</svelte:head>
<script lang="ts">
    import {browser} from "$app/env";
    import { faSearch } from '@fortawesome/pro-duotone-svg-icons';
    import TableData from 'svelte-bwen/components/TableData.svelte';
    import Input from 'svelte-bwen/components/form/Input.svelte';
    import Select from 'svelte-bwen/components/form/Select.svelte';
    import { getCountryFlagFromAlpha2 } from 'svelte-bwen/js/utils';
    import CountryFlag from "$lib/components/CountryFlag.svelte";
    import riso from "$lib/riso";

    let columns = [
        {key: "country", component: CountryFlag, componentProp: 'alpha2'},
        {key: "code", text: "Code"},
        {key: "parent", text: "parent"},
        {key: "name", text: "Name", },
        {key: "category", text: "category"},
    ];

    const filters = {
        search: '',
        country: '',
        parent: '',
    };

    let selectCountry = [];
    let selectParent = [];
    let filteredEntries = [];
    let entries = [];
    let optionsCountry = [];
    let optionsParent = [];
    let optionsFilteredParent = [];

    riso()
        .then((wasm) => {
            if (!browser) { return; }

            entries = wasm.iso3166.subdivisions();
            filterEntries(filters, entries);
        })
        // Little timeout not to slow down the first render of subdivisions datatable
        .then(() => setTimeout(populateFilters, 1));

    async function populateFilters() {
        if (!browser) { return; }

        let { iso3166 } = await riso();
        let optionsC = [];
        let countries = iso3166.countries();
        countries.forEach((country) => {
            optionsC.push({
                text: country.name,
                value: country.alpha2,
                unicode: getCountryFlagFromAlpha2(country.alpha2),
            });
        });

        let optionsP = [];
        let parents = iso3166.subdivisions_parents();
        parents.forEach((country) => {
            optionsP.push({
                text: country.name,
                value: country.code,
                unicode: getCountryFlagFromAlpha2(country.country),
            });
        });

        // Cant array.push(), does not trigger reactivity. Must re-assign whole array.
        optionsCountry = optionsC;
        optionsFilteredParent = optionsParent = optionsP;
    }

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
                matches &= entry.code.startsWith(filters.country);
            }

            if (filters.parent) {
                matches &= entry.parent === filters.parent;
            }

            return matches;
        });
    }

    function onSearch(event) {
        filters.search = event.target.value.toLowerCase();
        filterEntries(filters, entries);
    }

    function onFilterByCountry(event) {
        filters.country = event.detail;
        filterEntries(filters, entries);

        optionsFilteredParent = optionsParent.filter((entry) => {
            if (!filters.country) {
                return true;
            }

            return entry.value.startsWith(filters.country);
        });
    }

    function onFilterByParent(event) {
        filters.parent = event.detail;
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
        <Select name="country" bind:this={selectCountry} options={optionsCountry} placeholder="Filter by Country" on:change={onFilterByCountry} />
        <Select name="parent" bind:this={selectParent} options={optionsFilteredParent} placeholder="Filter by Parent" on:change={onFilterByParent} />
    </div>
    <TableData entries={filteredEntries} columns={columns} perPage=15 uid="code" />
</div>

