<script lang="ts">
    import { getCountryFlagFromAlpha2 } from 'svelte-bwen/js/utils';
    import { countryNames } from '$lib/stores/Iso3166Cache';
    import riso from "$lib/riso";

    export let alpha2;
    let alpha2s = [];
    $: {
        if (alpha2 && alpha2.match(/,/)) {
            alpha2s = alpha2.split(',');
        }
    }

    function getFlagTitle(iso3166, alpha2: string) {
        if (countryNames[alpha2]) {
            return alpha2 + ' - ' + countryNames[alpha2];
        }

        let country = iso3166.country_try_for(alpha2);
        countryNames[alpha2] = country.name;

        return alpha2 + ' - ' + countryNames[alpha2];
    }

    const promise = riso();
</script>

<style lang="css">
</style>

{#if alpha2 && alpha2s.length === 0}
    {#await promise}
        {alpha2}
    {:then { iso3166 }}
        <span class="country-flag" title={getFlagTitle(iso3166, alpha2)}>{getCountryFlagFromAlpha2(alpha2)}</span>
    {/await}
{:else if alpha2s.length >= 1}
    {#each alpha2s as alpha2}
        {#await promise}
            {alpha2}
        {:then { iso3166 }}
            <span class="country-flag" title={getFlagTitle(iso3166, alpha2)}>{getCountryFlagFromAlpha2(alpha2)}</span>
        {/await}
    {/each}
{/if}
