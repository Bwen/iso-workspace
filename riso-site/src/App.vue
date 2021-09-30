
<template>
  <div class="layout-wrapper">
      <LayoutHeader />
      <section>
        <router-view />
      </section>
  </div>
</template>

<script>
import LayoutHeader from './components/LayoutHeader.vue';

export default {
  name: 'App',
  components: {
    LayoutHeader,
  },
  data() {
    return {
      theme: 'saga-blue',
    }
  },
  created() {
    this.initTheme();
  },
  methods: {
    initTheme() {
        let appTheme = localStorage.getItem('theme') ?? 'saga-blue';
        const queryString = window.location.search;
        if (queryString) {
          appTheme = new URLSearchParams(queryString.substring(1)).get('theme');
        }

        this.changeTheme(appTheme);
    },
    changeTheme(theme) {
      let themeElement = document.getElementById('theme-link');
      themeElement.setAttribute('href', `themes/${theme}/theme.css`);
      localStorage.setItem('theme', theme);
    },
  }
}
</script>

<style>
body {
  margin: 0px;
  height: 100%;
  overflow-x: hidden;
  overflow-y: auto;
  background-color: var(--surface-a);
  font-family: var(--font-family);
  font-weight: normal;
  color: var(--text-color);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

div[class^="iso-"] {
  padding: 2rem;
  margin: auto;
  width: 80%;
}

div[class^="iso-"] .p-datatable td.flag {
  font-size: 2.5em;
  padding: 0;
}

div[class^="iso-"] .p-datatable .p-datatable-header .total {
    padding: 0;
    vertical-align: middle;
    line-height: 33px;
    font-size: 1.5em;
}

div[class^="iso-"] .p-datatable .p-datatable-header .total label {
  font-size: 0.65em;
}

</style>
