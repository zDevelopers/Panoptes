<template>

    <v-card elevation="2">
      <v-container>
        <v-row>
          <v-col cols="10">
            <PlayerStatsFilter v-model="statsFilter"></PlayerStatsFilter>
          </v-col>
          <v-col cols="2" align-self="center">
            <v-btn @click="onClickFilter">Filtrer</v-btn>
          </v-col>
        </v-row>
        <v-row>
          <v-col>
            <v-data-table no-data-text='Veuillez renseigner les filtres et cliquer sur "Filtrer"'
                          :items="dataStats" :headers="headers">

            </v-data-table>
          </v-col>
        </v-row>
        <v-row v-if="globalPlayerCount !== null">
          <p>Montant total des mouvements d'items : <span :class="globalPlayerCount >= 0 ? 'total-amount-ok' : 'total-amount-nok'">{{ globalPlayerCount }}</span></p>
        </v-row>
      </v-container>
    </v-card>
</template>

<script>
import PlayerStatsFilter from "@/components/filters/PlayerStatsFilter";
import ratioApi from "@/api/RatioApi";

export default {
  name: "PlayerTableStats",
  components: {PlayerStatsFilter},
  data: () => ({
    statsFilter: {players: [], areas: []},
    dataStats: [],
    globalPlayerCount: null
  }),
  computed: {
    headers: function()
    {
      return [
        {
          text: 'Item',
          align: 'start',
          sortable: true,
          value: 'item',
        },
        {
          text: 'Montant',
          align: 'start',
          sortable: true,
          value: 'amount',
        }
      ]
    }
  },
  watch: {
    statsFilter: function() {
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
      else
      {
        this.$store.commit("addError", {type: 'error'
          , description: 'Vous devez sélectionner au moins un joueur pour filtrer.'});
      }
    },
    resetRatio: function()
    {
      this.dataStats = []; // Remise à zéro du contenu de la datatable lors d'un changement de filtre
      this.globalPlayerCount = null;
    },
    refreshRatio: function()
    {
      ratioApi.getRatio(this.statsFilter.players, this.statsFilter.areas)
          .then(dataReturned => { this.showRatio(dataReturned.data); })
    },
    showRatio: function(ratiosData)
    {
      if(ratiosData.global)
      {
        this.globalPlayerCount = ratiosData.global;
      }

      if(ratiosData.detail)
      {
        this.dataStats = this.formatDetailRatioDatas(ratiosData.detail);
      }
      else
      {
        this.$store.commit("addError", {type: 'error', description:
              'Le détail des montants par bloc est absent'});
      }
    },
    formatDetailRatioDatas: function(detailDatas)
    {
      return Object.entries(detailDatas).map(detail => ({
        item: detail[0],
        amount: detail[1]
      }))
    }
  }
}
</script>

<style scoped>
.total-amount-ok
{
  color: green;
}

.total-amount-nok
{
  color: red;
  font-weight: bold;
}
</style>
