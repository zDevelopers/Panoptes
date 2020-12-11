import Vue from 'vue';

const areaApi = {
    list: function(filter)
    {
        return Vue.axios.get('areas');
    }
}

export default areaApi;
