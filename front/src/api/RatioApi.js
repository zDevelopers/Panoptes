import Vue from 'vue';

function _buildRatioParams(players, areas)
{
    let params = [];

    params.push({name: 'players', val: players.join(',')});

    if(areas.length && areas.length > 0)
    {
        params.push({name: 'areas', val: areas.join(',')});
    }

    let strParams = params.map(function(elem) {return `${elem.name}=${elem.val}`}).join('&');
    return `?${strParams}`;
}

const ratioApi = {
    getRatio: function(playerUuids, areaIds)
    {
        if(playerUuids.length === 0)
        {
            throw 'Vous devez renseigner au moins un joueur.';
        }

        let uri = _buildRatioParams(playerUuids, areaIds);
        //return Vue.axios.get('ratios' + uri);
        return Promise.resolve({ data:
            JSON.parse('{"users":[{"uuid":"","username":""},{"uuid":"","username":""}],"areas"' +
                ':["banque","point-c","ancienne-banque"],"global":-78444,"detail":{"minecraft:stone":-4877,"minecraft:cake":278}}')
        });
    }
}

export default ratioApi;
