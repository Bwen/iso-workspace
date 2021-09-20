<template>
    <span :title="getFlagTitle()">{{getFlagEmoji()}}<span v-if="this.withName">&nbsp;{{this.getCountry().name}}</span></span>
</template>
<script>
export default {
  name: 'CountryFlag',
  props: ['alpha2', 'withName', 'country'],
  data() {
      return {
          country_cache: null,
      };
  },
  methods: {
    getCountry() {
        if (this.country_cache &&
            (
                (this.alpha2 && this.country_cache.alpha2 === this.alpha2) ||
                (this.country && this.country_cache.alpha2 === this.country.alpha2)
            )
        ) {
            return this.country_cache;
        }

        if (this.country) {
            this.country_cache = this.country;
            return this.country;
        }

        this.country_cache = this.$riso3166.country_try_for(this.alpha2);
        return this.country_cache;
    },
    getFlagTitle() {
        if (this.withName) {
            return '';
        }

        const country = this.getCountry();
        if (country == null) {
            return '';
        }

        return `${country.alpha2} - ${country.name}`;
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
  },
}
</script>