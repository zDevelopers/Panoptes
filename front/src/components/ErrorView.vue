<template>
  <div id="error-view">
    <v-alert
        v-for="error in errors"
        :key="error.id"
        :type="error.type"
        :value="error.show"
        dismissible
        transition="scale-transition">
      {{ error.description }}
    </v-alert>
  </div>
</template>

<script>
import {mapState} from 'vuex';
export default {
  name: "ErrorView",
  computed: {
    ...mapState({
      errors: state => state.errors
    })

  }
}
</script>

<style scoped>
#error-view
{
  z-index: 99;
  position: absolute;
  left: 10px;
  top: 30px;
}
</style>
