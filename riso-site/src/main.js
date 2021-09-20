import {createApp} from 'vue';
import App from './App.vue';
import PrimeVue from 'primevue/config';

import 'primevue/resources/primevue.min.css'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css';
import router from './router';
import CountryFlag from '@/components/CountryFlag';
import Dropdown from 'primevue/dropdown';

const app = createApp(App).use(router);
(async () => {
    const { Riso3166, Riso639, Riso4217 } = await import('riso-wasm')
    app.config.globalProperties.$riso4217 = Riso4217;
    app.config.globalProperties.$riso3166 = Riso3166;
    app.config.globalProperties.$riso639 = Riso639;

    app.use(PrimeVue, { ripple: true });
    app.component('Dropdown', Dropdown);
    app.component('CountryFlag', CountryFlag);
    app.mount('#app');
})();
