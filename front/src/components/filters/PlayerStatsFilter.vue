<template>
  <div>
    <v-container>
      <v-row>
        <v-col>
          <PlayerFilter v-model="players"></PlayerFilter>
        </v-col>
        <v-col>
          <AreaFilter v-model="areas" ></AreaFilter>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script>

import PlayerFilter from "@/components/filters/PlayerFilter";
import AreaFilter from "@/components/filters/AreaFilter";
export default {
  name: "PlayerStatsFilter",
  props: ['value'],
  components: {AreaFilter, PlayerFilter},
  data: () => ({
  }),
  computed: {
    players: {
      get() {
        return this.value.players ? this.value.players : [];
      },
      set(val) {
        this.$emit('input', {players: val, areas: this.areas});
      }
    },
    areas: {
      get() {
        return this.value.areas ? this.value.areas : [];
      },
      set(val) {
        this.$emit('input', {players: this.players, areas: val});
      }
    },
    filter: function()
    {
      return {players: this.players, areas: this.areas};
    }
  }
}
</script>

<style scoped>

</style>
