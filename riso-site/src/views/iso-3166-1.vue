<template>
  <div class="iso-3166-1">
    <div class="p-grid">
      <div class="p-col p-text-right"><Button icon="pi pi-github" class="p-button-lg p-button-rounded p-button-text" :onclick="goToGithub" /></div>
    </div>

    <DataTable 
      @value-change="filtered"
      stripedRows
      responsiveLayout="scroll"
      dataKey="alpha2"
      v-model:expandedRows="expandedRows"
      :filters="filters"
      :value="countries"
      :paginator="true"
      :rows="10">

      <template #header>
        <div class="p-d-flex">
          <div class="p-mr-2">
            <span class="p-input-icon-left">
                <i class="pi pi-search" />
                <InputText v-model="filters['global'].value" placeholder="Keyword Search" />
            </span>
          </div>
          <div class="p-mr-2">
            <Checkbox id="details" v-model="displayDetails" :binary="true" />
            <label for="details">Display extra details</label>
          </div>
          <div class="total p-col p-text-right">
            <label>Total Entries:</label> {{ this.totalEntries }}
          </div>
        </div>
      </template>
      <Column field="alpha2" header="" class="flag p-text-center" style="width:60px;">
        <template #body="slotProps">
          <CountryFlag :alpha2="slotProps.data.alpha2"></CountryFlag>
        </template>
      </Column>
      <Column field="alpha2" header="Alpha 2" :sortable="true" class="p-text-center" style="width:120px;"></Column>
      <Column field="alpha3" header="Alpha 3" :sortable="true" class="p-text-center" style="width:120px;"></Column>
      <Column field="numeric" header="Numeric" :sortable="true" class="p-text-center" style="width:125px;"></Column>
      <Column field="name" header="Name" :sortable="true"></Column>
      <Column field="official_name" header="Official" :sortable="true"></Column>
      
      <Column :expander="true" v-if="displayDetails == 1" style="width: 32px;"></Column>
      <template #expansion="slotProps" v-if="displayDetails == 1">
          <div class="orders-subtable">
            <DataTable :value="[this.$riso3166.country_details_for(slotProps.data.alpha2)]" dataKey="alpha2" showGridlines>
              <Column field="tld" header="TLD" class="tld"></Column>
              <Column field="population" header="Population">
                <template #body="{ data }">{{ parseInt(data.population, 10).toLocaleString() }}</template>
              </Column>
              <Column field="phone_prefix" header="Phone Prefix"></Column>
              <Column field="currency" header="Currency"></Column>
              <Column field="continent" header="Continent">
                <template #body="{ data }">
                  {{ this.$riso3166.continent_try_for(data.continent).name }}
                </template>
              </Column>
              <Column field="languages" header="Languages"></Column>
              <Column field="neightboors" header="Neightboors" class="neightboors">
                <template #body="{ data }">
                  <span v-for="alpha2 in data.neightboors.split(',')" :key="alpha2">
                    <CountryFlag :alpha2="alpha2"></CountryFlag>
                  </span>
                </template>
              </Column>
            </DataTable>
          </div>
      </template>
    </DataTable>
  </div>
</template>

<script>
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import Checkbox from 'primevue/checkbox';
import DataTable from 'primevue/datatable'
import Column from 'primevue/column';
import { FilterMatchMode } from 'primevue/api';
import CountryFlag from '@/components/CountryFlag';

export default {
  name: 'Home',
  components: {
    DataTable,
    Column,
    Button,
    InputText,
    Checkbox,
    CountryFlag,
  },
  data() {
      return {
        totalEntries: 0,
        filters: {
          'global': {value: null, matchMode: FilterMatchMode.CONTAINS},
        },
        optionValue: null,
        continents: [],
        countries: [],
        displayDetails: 0,
        expandedRows: [],
      }
  },
  methods: {
    filtered(filteredValue) {
      this.totalEntries = filteredValue.length;
    },
    goToGithub() {
        window.open('https://github.com/Bwen/iso-workspace/tree/main/riso-3166', '_blank');
    },
  },
  mounted() {
    this.countries = this.$riso3166.countries();
    this.continents = this.$riso3166.continents();
  }
}
</script>

<style scoped>
.p-mr-2 [for="details"] {
  line-height: 32px;
}

.p-checkbox {
  vertical-align: middle;
  margin-right: 8px;
}

.iso-3166-1 >>> td {
  padding:0;
  line-height: 33px;
}
.iso-3166-1 >>> tr.p-datatable-row-expansion td {
  padding:0;
  line-height: 16px;
}
.iso-3166-1 >>> .p-datatable-thead .p-column-header-content {
  display: block;
}
.iso-3166-1 >>> tr.p-datatable-row-expansion .p-datatable .p-datatable-thead > tr > th[role=cell],
.iso-3166-1 >>> tr.p-datatable-row-expansion .p-datatable .p-datatable-tbody > tr > td[role=cell] {
  font-size: 0.75em;
  padding: 0.25em;
  text-align: center;
  color: var(--text-color-secondary);
}

.iso-3166-1 >>> tr.p-datatable-row-expansion .p-datatable .p-datatable-thead > tr > th[role=cell] {
  background-color: var(--surface-c);
}
.iso-3166-1 >>> tr.p-datatable-row-expansion .p-datatable .p-datatable-tbody > tr > td[role=cell] {
  background-color: var(--surface-d);
}

.iso-3166-1 >>> tr.p-datatable-row-expansion td.neightboors {
  margin:0;
}

.iso-3166-1 >>> tr.p-datatable-row-expansion td > span {
  font-size: 2.5em;
  padding: 0;
  margin-right: 3px;
}
</style>