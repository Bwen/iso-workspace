<template>
  <div class="iso-639">
    <div class="p-grid">
      <div class="p-col p-text-right"><Button icon="pi pi-github" class="p-button-lg p-button-rounded p-button-text" :onclick="goToGithub" /></div>
    </div>

    <DataTable 
      @value-change="filtered"
      stripedRows
      responsiveLayout="scroll"
      dataKey="alpha3"
      :value="languages"
      :filters="filters"
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
          <div class="total p-col p-text-right">
            <label>Total Entries:</label> {{ this.totalEntries }}
          </div>
        </div>
      </template>
      <Column field="alpha2" header="Alpha 2" :sortable="true"></Column>
      <Column field="alpha3" header="Alpha 3" :sortable="true"></Column>
      <Column field="text" header="Name" :sortable="true"></Column>
    </DataTable>
  </div>
</template>

<script>
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { FilterMatchMode } from 'primevue/api';
//import CountryFlag from '@/components/CountryFlag'

export default {
  name: 'Home',
  components: {
    DataTable,
    Column,
    Button,
    InputText,
    //CountryFlag,
  },
  data() {
      return {
        totalEntries: 0,
        filters: {
          'global': {value: null, matchMode: FilterMatchMode.CONTAINS},
        },
        languages: [],
      }
  },
  methods: {
    filtered(filteredValue) {
      this.totalEntries = filteredValue.length;
    },
    goToGithub() {
        window.open('https://github.com/Bwen/iso-workspace/tree/main/riso-639', '_blank');
    },
  },
  mounted() {
    this.languages = this.$riso639.languages()
  }
}
</script>

<style scoped>
</style>