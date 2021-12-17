<svelte:head>
    <title>RISO 4217</title>
    <meta name="description" content="ISO 4217 standard data from the Rust crate RISO-4217 as WASM pack" />
</svelte:head>
<script lang="ts">
    import {browser} from "$app/env";
    import { faSearch } from '@fortawesome/pro-duotone-svg-icons';
    import TableData from 'svelte-bwen/components/TableData.svelte';
    import { Input, Select } from 'svelte-bwen/components/form';
    import CountryFlag from "$lib/components/CountryFlag.svelte";
    import CurrencySymbol from "$lib/components/CurrencySymbol.svelte";
    import { getCountryFlagFromAlpha2 } from 'svelte-bwen/js/utils';
    import riso from "$lib/riso";

    let columns = [
        {key: "symbol", component: CurrencySymbol, componentProp: 'symbol'},
        {key: "code", text: "Code"},
        {key: "units", text: "Units"},
        {key: "name", text: "Name", },
        {key: "countries", component: CountryFlag, componentProp: 'alpha2'},
    ];

    const filters = {
        search: '',
        country: '',
    };

    let optionsCountry = [];
    let selectCountry = [];
    let filteredEntries = [];
    let entries = [];
    riso()
        .then((wasm) => {
            if (!browser) { return; }

            entries = wasm.iso4217.currencies();
            filteredEntries = entries;
        })
        // Little timeout not to slow down the first render of currencies datatable, and avoid wasm double load
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

        // Cant array.push(), does not trigger reactivity. Must re-assign whole array.
        optionsCountry = optionsC;
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
                matches &= entry.countries.includes(filters.country);
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
    </div>
    <TableData entries={filteredEntries} columns={columns} perPage=15 uid="alpha2" />
</div>
