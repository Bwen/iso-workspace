<svelte:head>
    <title>RISO Demo Site</title>
    <meta name="description" content="Demo site for the RISO Rust crate of ISO-3166, ISO-639, ISO-4217 standards as web assembly" />
</svelte:head>

<style lang="css">
</style>
<script lang="ts">
    import { Heading, Link } from 'svelte-bwen/components';
    import { faRust, faGithub } from '@fortawesome/free-brands-svg-icons';
    import { faLanguage } from '@fortawesome/free-solid-svg-icons';
    import { faGlobe } from '@fortawesome/pro-light-svg-icons';
    import { faCoins } from '@fortawesome/pro-duotone-svg-icons';
</script>

<div class="page">
    <Heading tag="h1" text="RISO Demo Site" />
    <p>
        RISO, which stands for <q>Rust ISO</q>, is a collection of <Link icon={faRust} href="https://www.rust-lang.org/">Rust</Link>
        libraries hosted on <Link href="https://crates.io/search?q=riso">crates.io</Link>. The goal of these libraries is to
        offer a convenient way to access the static data of the ISO standards while having a process in place to keep them
        up to date on a periodic basis. RISO only covers 3 standards: ISO-3166, ISO-4217 & ISO-639, but may increase its
        coverage in the future.
    </p>

    <Heading tag="h2" text="ISO Standards" />
    <p>
        <Link href="https://iso.org">The International Organization for Standardization</Link> is independent and non-governmental,
        they approve international standards and assign
        <Link href="https://www.iso.org/maintenance_agencies.html">maintenance agencies and registration authorities</Link> to each
        standard.
    </p>
    <figure>
        <blockquote cite="https://www.iso.org/about-us.html">
            Because 'International Organization for Standardization' would have different acronyms in different languages
            (IOS in English, OIN in French for Organisation internationale de normalisation), our founders decided to
            give it the short form ISO. ISO is derived from the Greek 'isos', meaning equal. Whatever the country,
            whatever the language, we are always ISO.
        </blockquote>
        <figcaption>~ International Organization for Standardization</figcaption>
    </figure>
    <p>If you would like to know more about them, see their <Link href="https://www.iso.org/about-us.html">about page</Link>.</p>

    <p>
        The data exposed by these libraries is in accordance with the
        <Link href="https://www.iso.org/maintenance_agencies.html">maintenance agencies and registration authorities</Link>
        assigned by the ISO. The extra details are not officially assigned to <Link href="http://www.geonames.org/about.html">GeoNames</Link>
        by ISO, but its a great source of data that is kept up to date on a regular basis. Only information not already
        provided by ISO is taken from <Link href="http://www.geonames.org/about.html">GeoNames</Link>.
    </p>

    <Heading tag="h3" icon={faGlobe} subtext="Countries & Subdivisions" text="ISO-3166" />
    <p>
        Library Source: <Link icon={faGithub} href="https://github.com/Bwen/iso-workspace/tree/main/riso-3166">iso-workspace/tree/main/riso-3166</Link><br />
        Data Source: <Link href="https://www.iso.org/mara/iso3166">https://www.iso.org/mara/iso3166</Link><br />
        Details Source: <Link href="http://www.geonames.org/export/">http://www.geonames.org/export/</Link>
    </p>

    <Heading tag="h3" icon={faCoins} subtext="Currencies" text="ISO-4217" />
    <p>
        Library Source: <Link icon={faGithub} href="https://github.com/Bwen/iso-workspace/tree/main/riso-4217">iso-workspace/tree/main/riso-4217</Link><br />
        Data Source: <Link href="https://www.six-group.com/en/products-services/financial-information/data-standards.html">https://www.six-group.com/en/products-services/financial-information/data-standards.html</Link>
    </p>

    <Heading tag="h3" icon={faLanguage} subtext="Languages" text="ISO-639" />
    <p>
        Library Source: <Link icon={faGithub} href="https://github.com/Bwen/iso-workspace/tree/main/riso-639">iso-workspace/tree/main/riso-639</Link><br />
        Data Source: <Link href="https://www.loc.gov/standards/iso639-2">https://www.loc.gov/standards/iso639-2</Link>
    </p>
</div>
