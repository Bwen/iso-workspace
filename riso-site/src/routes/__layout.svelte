<script lang="ts">
    import 'svelte-bwen/js/media-theme';
    import 'svelte-bwen/css/sanitize.scss';
    import 'svelte-bwen/css/themes/default/index.css';
    import 'svelte-bwen/css/themes/material/index.css';
    import '$lib/css/global.css';

    import { faLanguage, faHome } from '@fortawesome/free-solid-svg-icons';
    import { faGlobe } from '@fortawesome/pro-light-svg-icons';
    import { faGithub } from '@fortawesome/free-brands-svg-icons';
    import { faCoins, faLightbulbSlash, faLightbulbOn } from '@fortawesome/pro-duotone-svg-icons';
    import { AppHeader } from 'svelte-bwen/components';
    import type { LinkDefinition } from 'svelte-bwen/js/types';

    let centerItems: LinkDefinition[] = [
        {icon: faLanguage, text: "ISO-639", subtext: "Languages", href: "/iso-639"},
        {icon: faGlobe, text: "ISO-3166-1", subtext: "Countries", href: "/iso-3166-1"},
        {icon: faGlobe, text: "ISO-3166-2", subtext: "Subdivisions", href: "/iso-3166-2"},
        {icon: faCoins, text: "ISO-4217", subtext: "Currencies", href: "/iso-4217"},
    ];

    let rightItems: LinkDefinition[] = [
        {icon: faGithub, href: "https://github.com/Bwen/iso-workspace"},
        {icon: faLightbulbOn, icon_hover: faLightbulbSlash, id: "light-dark-switch"},
    ];

    function onItemClick(event) {
        if ('light-dark-switch' === event.detail.hyperlink.id) {
            let currentTheme = document.documentElement.getAttribute('theme');
            if (!currentTheme) {
                console.warn('TODO: Implement theme switcher');
                return;
            }

            let changeTo = `${currentTheme}-dark`;
            if (currentTheme.match(/-dark/)) {
                changeTo = currentTheme.replace(/-dark/, '');
            }

            document.documentElement.setAttribute('theme', changeTo);
            localStorage.setItem('theme', changeTo);
        }
    }

    setTimeout(() => {
        // Hack so the child component prop is reactive on array length...
        rightItems[0].icon = rightItems[0].icon;

        rightItems.push({text: "🤔💭", href: "/about"});
    }, 180000); // 3 Minutes
</script>

<style lang="css">
</style>

<AppHeader logoItem={{icon: faHome, href: "/"}} centerItems={centerItems} rightItems={rightItems} on:item-click={onItemClick}/>
<main>
    <slot></slot>
</main>
<br />
