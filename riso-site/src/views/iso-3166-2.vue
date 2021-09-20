<template>
  <div class="iso-3166-2">
    <div class="p-grid">
      <div class="p-col p-text-right"><Button icon="pi pi-github" class="p-button-lg p-button-rounded p-button-text" :onclick="goToGithub" /></div>
    </div>

    <DataTable @value-change="filtered" :value="subdivisions" stripedRows responsiveLayout="scroll" dataKey="code" :filters="filters" :paginator="true" :rows="10">
      <template #header>
        <div class="p-d-flex">
          <div class="p-mr-2">
            <span class="p-input-icon-left">
                <i class="pi pi-search" />
                <InputText v-model="filters['name'].value" placeholder="Name search" />
            </span>
          </div>
          <div class="p-mr-2">
            <CountryDropdown 
              @change="filterChange" 
              @filter-change="filterChange" 
              optionValue="alpha2" 
              :event-change="eventChange" 
              :filter="filters['country']">
            </CountryDropdown>
          </div>
          <div class="p-mr-2">
            <SubdivisionParentDropdown 
              @change="filterChange" 
              @filter-change="filterChange" 
              optionValue="code"
              :event-change="eventChange" 
              :filter="filters['parent']">
            </SubdivisionParentDropdown>
          </div>
          <div class="total p-col p-text-right">
            <label>Total Entries:</label> {{ this.totalEntries }}
          </div>
        </div>
      </template>
      <Column field="country" header="Country" style="width: 100px;" class="flag p-text-center">
        <template #body="slotProps">
            <CountryFlag :alpha2="slotProps.data.country"></CountryFlag>
        </template>
      </Column>
      <Column field="code" header="Code"></Column>
      <Column field="parent" header="Parent"></Column>
      <Column field="name" header="Name"></Column>
      <Column field="category" header="Category"></Column>
    </DataTable>
  </div>
</template>

<script>
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { FilterMatchMode } from 'primevue/api'
import CountryDropdown from '@/components/CountryDropdown'
import SubdivisionParentDropdown from '@/components/SubdivisionParentDropdown'

export default {
  name: 'Home',
  components: {
    DataTable,
    Column,
    Button,
    InputText,
    CountryDropdown,
    SubdivisionParentDropdown,
  },
  data() {
      return {
        totalEntries: 0,
        filters: {
          'name': {value: null, matchMode: FilterMatchMode.CONTAINS},
          'country': {value: null, matchMode: FilterMatchMode.EQUALS},
          'parent': {value: null, matchMode: FilterMatchMode.EQUALS},
        },
        eventChange: {},
        subdivisions: [],
      }
  },
  methods: {
    filterChange(event) {
      this.eventChange = event;
    },
    filtered(filteredValue) {
      this.totalEntries = filteredValue.length;
    },
    goToGithub() {
      window.open('https://github.com/Bwen/iso-workspace/tree/main/riso-3166', '_blank');
    },
  },
  mounted() {
    this.subdivisions = this.$riso3166.subdivisions();
  }
}
</script>

<style scoped>
</style>