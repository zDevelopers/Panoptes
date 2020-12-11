<template>
  <v-autocomplete :loading="loading" :items="items" v-model="areas" multiple label="Zones"></v-autocomplete>
</template>

<script>
import areaApi from "@/api/AreaApi"

export default {
  name: "AreaFilter",
  props: ['value'],
  data : () => ({
    items: [],
    loading: true
  }),
  computed: {
    areas: {
      get() {
        return this.value;
      },
      set(val) {
        this.$emit('input', val);
      }
    }
  },
  methods: {
    getAreaList: function()
    {
      areaApi.list().then(data => {
        this.items = this.formatAreaList(data.data);
      }).finally(() => {
        this.loading = false;
      })
    },
    formatAreaList: function(data)
    {
      return data.map(aData => ({text: aData.name, value: aData.id}));
    }
  },
  mounted() {
    this.getAreaList();
  }
}
</script>

<style scoped>

</style>
