<template>
    <span>
        <Dropdown  @change="change" :optionValue="this.dropdownValue" v-model="this.parentFilter.value" :options="this.subdivisions" optionLabel="name" :filter="true" placeholder="Select a Parent Subdivision" :showClear="true">
            <template #value="slotProps">
                <div class="p-dropdown-subparent-value" v-if="slotProps.value">
                    <CountryFlag :alpha2="slotProps.value.split('-')[0]"></CountryFlag>
                    {{slotProps.value}} {{getParentName(slotProps.value)}}
                </div>
                <span v-else>
                    {{slotProps.placeholder}}
                </span>
            </template>
            <template #option="slotProps">
                <div class="p-dropdown-subparent-option">
                    <CountryFlag :alpha2="slotProps.option.country"></CountryFlag>
                    {{slotProps.option.code}} {{slotProps.option.name}}
                </div>
            </template>
        </Dropdown>
    </span>
</template>
<script>
import Dropdown from 'primevue/dropdown'
import CountryFlag from '@/components/CountryFlag'

export default {
  name: 'SubdivisionParentDropdown',
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
    getParentName(code) {
      let parent = this.$riso3166.subdivision_try_for(code);
      if (parent == null) {
        return '';
      }

      return parent.name;
    },
    getFlagEmoji() {
        const country = this.getCountry();
        if (country == null) {
            return '';
        }

        const codePoints = country.alpha2
            .toUpperCase()
            .split('')
            .map(char =>  127397 + char.charCodeAt());

        return String.fromCodePoint(...codePoints);
    },
    change(event) {
      if (event.value) {
        let subdiv = this.$riso3166.subdivision_try_for(event.value);
        if (subdiv !== null && subdiv.parent !== null) {
          this.countryFilter = subdiv.code.split('-')[0];
          this.subdivisions = this.$riso3166.subdivisions_parents_country(this.countryFilter);
        }
      } else if (this.countryFilter !== null) {
        this.subdivisions = this.$riso3166.subdivisions_parents_country(this.countryFilter);
      } else {
          this.subdivisions = this.$riso3166.subdivisions_parents(); 
      }

      event.type = 'SubdivisionParentDropdown';
      this.$emit('change', event);
    }
  },
  watch: {
      eventChange: function (event) {
        if (event.type == 'CountryDropdown') {
          this.parentFilter.value = null;

          if (event.value && this.parentFilter.value != event.value) {
            this.countryFilter = event.value;
            this.subdivisions = this.$riso3166.subdivisions_parents_country(event.value);
          } else {
            this.countryFilter = null;
            this.subdivisions = this.$riso3166.subdivisions_parents(); 
          }
        }
      }
  },
  data() {
      return {
          dropdownValue: this.optionValue,
          parentFilter: this.filter,
          countryFilter: null,
          subdivisions: null,
      };
  },
  mounted() {
    this.subdivisions = this.$riso3166.subdivisions_parents();    
  },
}
</script>