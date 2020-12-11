<template>
  <v-app>
    <v-app-bar
      id="panoptes-app-bar"
      app
      dark
    >
      <div class="d-flex align-center">
        <v-img
          alt="Vuetify Logo"
          class="shrink mr-2"
          contain
          src="../src/assets/logo.png"
          transition="scale-transition"
          width="40"
        />

        <span id="panoptes-title">Panoptès</span>
      </div>

      <v-spacer></v-spacer>

      <v-btn
        href="https://zcraft.fr"
        target="_blank"
        text
      >
        <span class="mr-2">Retourner sur Zcraft</span>
        <v-icon>mdi-open-in-new</v-icon>
      </v-btn>
    </v-app-bar>
    <v-main>
      <ErrorView></ErrorView>
      <div class="player-table-stats">
        <PlayerTableStats></PlayerTableStats>
      </div>
    </v-main>
  </v-app>
</template>

<script>
import PlayerTableStats from "@/components/PlayerTableStats";
import ErrorView from "@/components/ErrorView";

export default {
  name: 'App',

  components: {
    ErrorView,
    PlayerTableStats
  }
};
</script>

<style scoped>
#panoptes-title
{
  font-size: 1.8rem;
}
#panoptes-app-bar
{
  border-bottom-color: green;
  border-bottom-width: 2px;
  border-bottom-style: solid;
}
.player-table-stats
{
  margin: 20px;
}
</style>
