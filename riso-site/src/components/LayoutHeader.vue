<template>
    <div class="layout-topbar p-shadow-2">
        <Menubar :model="items">
            <template #start>
                <router-link to="/"><img alt="logo" src="https://www.primefaces.org/wp-content/uploads/2020/05/placeholder.png" height="40" class="p-mr-2"></router-link>
            </template>
            <template #end>
                <Button icon="pi pi-github" class="p-button-lg p-button-rounded p-button-text" :onclick="goToGithub" />
                <ToggleButton v-model="checked" onIcon="pi pi-moon" offIcon="pi pi-sun" :change="changeTheme()" />
            </template>
        </Menubar>
    </div>
</template>

<script>
import Menubar from 'primevue/menubar';
import ToggleButton from 'primevue/togglebutton';
import Button from 'primevue/button';

export default {
    name: 'LayoutHeader',
    components: {
        Menubar,
        ToggleButton,
        Button,
    },
    methods: {
        goToGithub() {
            window.open('https://github.com/Bwen/iso-workspace', '_blank');
        },
        changeTheme() {
            let theme = this.checked ? 'saga-blue' : 'vela-blue';
            let themeElement = document.getElementById('theme-link');
            themeElement.setAttribute('href', `themes/${theme}/theme.css`);
            localStorage.setItem('theme', theme);
        },
    },
    data() {
        return {
            checked: localStorage.getItem('theme') === 'saga-blue',
            items: [
                {
                    label:'ISO-639',
                    icon:'pi pi-fw pi-globe',
                    to: '/iso-639',
                },
                {
                    label:'ISO-3166-1',
                    icon:'pi pi-fw pi-map',
                    to: '/iso-3166-1',
                },
                {
                    label:'ISO-3166-2',
                    icon:'pi pi-fw pi-map',
                    to: '/iso-3166-2',
                },
                {
                    label:'ISO-4217',
                    icon:'pi pi-fw pi-dollar',
                    to: '/iso-4217',
                },
            ]
        }
    },
    props: {
    },
}
</script>

<style>
.layout-topbar .p-menubar {
  border-width: 0 0 1px 0;
  border-radius: 0;
  padding: 0;
  background-color: var(--surface-c);
}

.layout-topbar .p-menubar .p-menubar-root-list > .p-menuitem > .p-menuitem-link {
  border-radius: 0;
  height: 50px;
}

.layout-topbar .p-menubar .p-menubar-root-list > .p-menuitem > .p-menuitem-link.router-link-active {
  background-color: var(--surface-d);
  border-style: solid;
  border-width: 0 1px;
  border-color: var(--surface-b);
}

.layout-topbar .p-button.p-togglebutton.p-component.p-button-icon-only {
  background-color: var(--gray-300);
}

.layout-topbar .p-button.p-togglebutton.p-component.p-button-icon-only .p-button-icon {
  color: var(--yellow-200);
}

.layout-topbar .p-button.p-togglebutton.p-component.p-button-icon-only.p-highlight {
  background-color: var(--gray-800);
}

.layout-topbar .p-button.p-togglebutton.p-component.p-button-icon-only.p-highlight .p-button-icon {
  color: white;
}

</style>