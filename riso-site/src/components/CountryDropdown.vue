<template>
    <span>
        <Dropdown  @change="change" :optionValue="this.dropdownValue" v-model="this.countryFilter.value" :options="this.countries" optionLabel="name" :filter="true" placeholder="Select a Country" :showClear="true">
            <template #value="slotProps">
                <div class="p-dropdown-country-value" v-if="slotProps.value">
                    <CountryFlag :alpha2="slotProps.value" with-name=true></CountryFlag>
                </div>
                <span v-else>
                    {{slotProps.placeholder}}
                </span>
            </template>
            <template #option="slotProps">
                <div class="p-dropdown-country-option">
                    <CountryFlag :country="slotProps.option" with-name=true></CountryFlag>
                </div>
            </template>
        </Dropdown>
    </span>
</template>
<script>
import Dropdown from 'primevue/dropdown'
import CountryFlag from '@/components/CountryFlag'

export default {
  name: 'CountryDropdown',
  props: {
    filter: {
      type: Object,
      required: true
    },
    optionValue: {
      type: String,
      required: true
    },
    eventChange: {
      type: Object,
      required: false
    },
  },
  components: [
    Dropdown,
    CountryFlag,
  ],
  methods: {
      change(event) {
        event.type = 'CountryDropdown';
        this.$emit('change', event);
      }
  },
  watch: {
      eventChange: function (event) {
        if (event.type == 'SubdivisionParentDropdown') {
            if (event.value && this.countryFilter.value != event.value) {
                const country_alpha2 = event.value.split('-')[0];
                this.countryFilter.value = country_alpha2;
            }
        }
      }
  },
  data() {
      return {
          dropdownValue: this.optionValue,
          countryFilter: this.filter,
          countries: null,
      };
  },
  mounted() {
    this.countries = this.$riso3166.countries();    
  },
}
</script>