import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    errors: [],
    errorIdsCount: 0
  },
  mutations: {
    addError(state, payload)
    {
      state.errors.push({id: state.errorIdsCount, type: payload.type, description: payload.description, show: true});
      state.errorIdsCount++;
    },
    removeError(state, id)
    {
      let index = state.errors.findIndex(elem => elem.id === id);
      state.errors.splice(index, 1);
    }
  },
  actions: {
  },
  modules: {
  }
});

export default store;
