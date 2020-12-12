<template>
  <v-autocomplete :loading="loading" :items="items" cache-items :search-input.sync="textSearch"
                  v-model="player" multiple label="Joueurs"></v-autocomplete>
</template>

<script>
import playerApi from "@/api/PlayerApi";
export default {
name: "PlayerFilter",
  props: ['value'],
  data : () => ({
    items: [],
    loading: true,
    textSearch: ''
  }),
  computed: {
    player: {
      get() {
        return this.value;
      },
      set(val) {
        this.$emit('input', val);
      }
    }
  },
  methods: {
    getPlayerList: function()
    {
      playerApi.list(this.textSearch).then(data => {
        this.items = this.formatPlayerList(data.data);
      }).finally(() => {
        this.loading = false;
      })
    },
    formatPlayerList: function(data)
    {
      return data.map(jData => ({text: jData.name, value: jData.uuid}));
    }
  },
  watch: {
    textSearch: function(newVal, oldVal)
    {
      if(!this.loading)
      {
        this.getPlayerList();
      }
    }
  },
  mounted() {
    this.getPlayerList();
  }
}
</script>

<style scoped>

</style>
