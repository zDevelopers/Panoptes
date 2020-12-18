<template>
    <v-card elevation="2">
      <v-container>
        <v-row>
          <v-col class="col-12 col-sm-10">
            <PlayerStatsFilter :players.sync="statsFilter.players" :areas.sync="statsFilter.areas"></PlayerStatsFilter>
          </v-col>
          <v-col class="col-12 col-sm-2" align-self="center">
            <v-btn @click="onClickFilter" :disabled="!isPlayersNotEmpty">Filtrer</v-btn>
          </v-col>
        </v-row>
        <v-row v-if="dataStats.length > 0 || loading">
          <v-col cols="4" class="col-12 col-sm-4">
            <v-text-field
                v-model="search"
                append-icon="mdi-magnify"
                label="Filtrer les éléments…"
                single-line
                hide-details
                class="mb-4"
            ></v-text-field>
            <v-card elevation="4" outlined shaped class="summary">
              <v-card-text v-if="loading">
                <v-skeleton-loader
                    type="text,chip"
                ></v-skeleton-loader>
              </v-card-text>
              <v-list-item three-line v-else>
                <v-list-item-content>
                  <div class="overline">
                    Résumé des mouvements
                  </div>
                  <v-list-item-title class="total-amount" :class="globalPlayerCount >= 0 ? 'is-ok' : 'is-nok'">
                    {{ globalRatio }}
                  </v-list-item-title>
                  <v-list-item-subtitle>Somme des dépôts et des retraits</v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
            </v-card>
          </v-col>
          <v-col>
            <v-data-table
                :loading="loading"
                loading-text="Chargement en cours… veuillez patienter."
                :items="dataStats"
                :headers="headers"
                :search="search"
                :custom-filter="filterData"
                class="players-ratios-result">
              <template v-slot:item.item="{ item }">
                <div class="minecraft-material">
                  <p>{{ item.item.display_name }}</p>
                  <aside>{{ item.item.id }}</aside>
                </div>
              </template>
              <template v-slot:item.ratio="{ item }">
                <span class="ratio-pill" :class="{ 'is-ok': item.ratio > 0, 'is-nok': item.ratio < 0 }">
                  {{ format(item.ratio) }}
                </span>
              </template>
            </v-data-table>
          </v-col>
        </v-row>
      </v-container>
    </v-card>
</template>

<script>
import PlayerStatsFilter from "@/components/filters/PlayerStatsFilter";
import ratioApi from "@/api/RatioApi";

const formatter = new Intl.NumberFormat('fr-FR');

export default {
  name: "PlayerTableStats",
  components: {PlayerStatsFilter},
  data: () => ({
    statsFilter: {players: [], areas: []},
    dataStats: [],
    globalPlayerCount: null,
    loading: false,
    search: '',
  }),
  computed: {
    isPlayersNotEmpty: function()
    {
      return this.statsFilter.players.length > 0;
    },
    headers: function()
    {
      return [
        {
          text: 'Objet',
          align: 'start',
          sortable: false,
          value: 'item',
        },
        {
          text: 'Ratio',
          align: 'center',
          sortable: true,
          value: 'ratio',
        }
      ]
    },
    globalRatio: function() {
      return formatter.format(this.globalPlayerCount)
    }
  },
  watch: {
    statsFilter: function()
    {
      this.resetRatio();
    }
  },
  methods: {
    onClickFilter: function()
    {
      if(this.statsFilter.players.length > 0)
      {
        this.refreshRatio();
      }
    },
    resetRatio: function()
    {
      this.dataStats = []; // Remise à zéro du contenu de la datatable lors d'un changement de filtre
      this.globalPlayerCount = null;
    },
    refreshRatio: function()
    {
      this.loading = true;
      ratioApi.getRatio(this.statsFilter.players, this.statsFilter.areas)
          .then(dataReturned =>{ this.showRatio(dataReturned.data); })
          .finally(() => { this.loading = false; });
    },
    showRatio: function(ratiosData)
    {
      if(ratiosData.global)
      {
        this.globalPlayerCount = ratiosData.global;
      }

      if(ratiosData.detail)
      {
        this.dataStats = this.formatDetailRatioData(ratiosData.detail);
      }
      else
      {
        this.$store.commit("addError", {type: 'error', description:
              'Le détail des montants par bloc est absent'});
      }
    },
    formatDetailRatioData: function(detailData)
    {
      return detailData.map(detail => ({
        item: {
          id: detail.id,
          display_name: detail.display_name
        },
        ratio: detail.ratio
      }))
    },
    filterData: function(value, search) {
      if (typeof value == 'number') return false
      let normalizedSearch = search.trim().toLocaleLowerCase();
      return value.id.toLowerCase().replace("minecraft:", "").includes(normalizedSearch)
          || value.display_name.toLocaleLowerCase().includes(normalizedSearch)
    },
    format: function(value) {
      return formatter.format(value)
    }
  }
}
</script>

<style lang="sass">
.players-ratios-result
  @media screen and (min-width: 600px)
    td:last-child, th:last-child
      width: 20%

  .minecraft-material
    padding: .4rem 0
    p
      margin: 0
      font-size: 1.04rem
      font-weight: bold
    aside
      font-family: "JetBrains Mono", "Fira Code", "Consolas", monospace
      color: hsl(209, 14%, 37%)

  .ratio-pill
    --ratio-pill-background: hsl(214, 15%, 91%)
    --ratio-pill-color: hsl(210, 24%, 16%)

    display: inline-block
    padding: .2rem 1rem

    border-radius: 16px

    background-color: var(--ratio-pill-background)
    color: var(--ratio-pill-color)

    &.is-ok
      --ratio-pill-background: hsl(83, 88%, 94%)
      --ratio-pill-color: hsl(81, 86%, 14%)

    &.is-nok
      --ratio-pill-background: hsl(360, 100%, 97%)
      --ratio-pill-color: hsl(360, 92%, 20%)

.v-card.summary
  .total-amount
    font-size: 3rem
    font-weight: 100

    &.is-ok
      color: green

    &.is-nok
      color: red
</style>
