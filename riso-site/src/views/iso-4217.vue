<template>
  <div class="iso-4217">
    <div class="p-grid">
      <div class="p-col p-text-right"><Button icon="pi pi-github" class="p-button-lg p-button-rounded p-button-text" :onclick="goToGithub" /></div>
    </div>

    <DataTable @value-change="filtered" :value="currencies" stripedRows responsiveLayout="scroll" dataKey="alpha2" :filters="filters" :paginator="true" :rows="10">
      <template #header>
        <div class="p-d-flex">
          <div class="p-mr-2">
            <span class="p-input-icon-left">
                <i class="pi pi-search" />
                <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
            </span>
          </div>
          <div class="p-mr-2">
            <CountryDropdown optionValue="alpha2" :filter="filters['countries']"></CountryDropdown>
          </div>
          <div class="total p-col p-text-right">
            <label>Total Entries:</label> {{ this.totalEntries }}
          </div>
        </div>
      </template>
      <Column field="symbol" header="Sym." style="width: 67px;" class="symbol p-text-center"></Column>
      <Column field="code" header="Code" style="width: 67px;" class="p-text-center"></Column>
      <Column field="numeric" header="Num." style="width: 67px;" class="p-text-center"></Column>
      <Column field="units" header="units" style="width: 67px;" class="p-text-center"></Column>
      <Column field="name" header="Name"></Column>
      <Column field="countries" header="Countries" class="flag p-text-center">
        <template #body="slotProps">
          <ul class="flags">
            <li v-for="alpha2 in slotProps.data.countries.split(',')" :key="alpha2">
              <CountryFlag :alpha2="alpha2"></CountryFlag>
            </li>
          </ul>
        </template>
      </Column>
    </DataTable>
  </div>
</template>

<script>
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { FilterMatchMode } from 'primevue/api'
import CountryFlag from '@/components/CountryFlag'
import CountryDropdown from '@/components/CountryDropdown'

export default {
  name: 'Home',
  components: {
    DataTable,
    Column,
    Button,
    InputText,
    //Dropdown,
    CountryFlag,
    CountryDropdown,
  },
  data() {
      return {
        totalEntries: 0,
        filters: {
          'global': {value: null, matchMode: FilterMatchMode.CONTAINS},
          'countries': {value: null, matchMode: FilterMatchMode.CONTAINS},
        },
        currencies: [],
        //countries: null,
      }
  },
  methods: {
    filtered(filteredValue) {
      this.totalEntries = filteredValue.length;
    },
    goToGithub() {
      window.open('https://github.com/Bwen/iso-workspace/tree/main/riso-4217', '_blank');
    },
  },
  mounted() {
    this.currencies = this.$riso4217.currencies();
    this.totalEntries = this.currencies.length;
  }
}
</script>

<style scoped>

.iso-4217 ul.flags {
  margin: 0;
  padding: 0;
  list-style-type: none;
  display: inline-block;
}

.iso-4217 ul.flags >>> li {
  margin: 0 0 0 5px;
  padding: 0;
  display: inline-block;
  cursor: default;
}

.iso-4217 >>> td.symbol {
  font-size: 2em;
  padding: 0;
}
</style>