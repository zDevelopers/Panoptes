<template>
  <div>
    <v-container>
      <v-row>
        <v-col>
          <PlayerFilter :value="players" @input="emitPropChange('players', $event)"></PlayerFilter>
        </v-col>
        <v-col>
          <AreaFilter :value="areas" @input="emitPropChange('areas', $event)"></AreaFilter>
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
  props: {
    players: { default: () => ([]) },
    areas: { default: () => ([]) },
  },
  components: {AreaFilter, PlayerFilter},
  methods: {
    emitPropChange(propName, value)
    {
      this.$emit(`update:${propName}`, value);
    }
  }
}
</script>

<style scoped>

</style>
