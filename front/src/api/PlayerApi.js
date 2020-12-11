import Vue from 'vue';

const playerApi = {
    list: function(filter)
    {
        let strFilter = filter ? '?filter=' + filter : '';
        return Vue.axios.get('players' + strFilter);
        //return Promise.resolve(JSON.parse('[{"name":"moribus","uuid":"e4953e0c-eaff-4aaf-a597-d2a7794b1684"},{"name":"MrDomoo","uuid":"6979bcf4-a0da-46fc-89ed-24c40d3b0ab0"},{"name":"BsamohT","uuid":"f5494a6b-f86a-43bc-a180-144c24f408b8"}]'));
    }
}

export default playerApi;
