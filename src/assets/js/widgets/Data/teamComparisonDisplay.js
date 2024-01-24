function addTeamToCompare(event, data) {
    let teamNumber = event.srcElement.value;
    let team = data[teamNumber];

    let card = `
    <div class="team-entry">
    <div class="label">${team.team_number}</div>
    <div class="team-name">${(team.tba_data) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
    <table class="cube-table">
      <tr>
        <th>Low</th>
        <th>Medium</th>
        <th>High</th>
      </tr>
      <tr>
        <td>${round2Two(team.summary.avg_low)}±${round2Two(team.summary.low_confidence)}</td>
        <td>${round2Two(team.summary.avg_med)}±${round2Two(team.summary.med_confidence)}</td>
        <td>${round2Two(team.summary.avg_high)}±${round2Two(team.summary.high_confidence)}</td>
      </tr>

    </table>
    <table class="cube-table">
      <tr>
        <th>Teleop Points</th>
        <th>Auto Points</th>
      </tr>
      <tr>
        <td>${round2Two(team.summary.teleop_points)}±${round2Two(team.summary.teleop_point_confidence)}</td>
        <td>${round2Two(team.summary.auto_points)}±${round2Two(team.summary.auto_point_confidence)}</td>
      </tr>
    </table>
    <table class="station-table">
      <caption>Charge Station</caption>
      <tr>
        <th>Can Balance</th>
        <th>Docked %</th>
        <th>Engaged %</th>
      </tr>
      <tr>
        <td>${team.summary.can_balance}</td>
        <td>${round2Two(team.summary.balance_percentage*100)}</td>
        <td>${round2Two(team.summary.dock_percentage*100)}</td>
      </tr>
    </table>
    </div>`;

    document.querySelector(".comparison-team-cards").innerHTML += card;


}

function initializeTeamComparisons(event) {
    let data = event.detail.scout_data;
    let teamNumbers = Object.keys(data);

    let teamOptionsHtml = "<option value='' selected disabled hidden>Team</option>"
    teamNumbers.forEach(teamNumber => {
        
        
        teamOptionsHtml += `<option value=${teamNumber}>${teamNumber}</option>`
        
        
    })
    let select =document.querySelector(".comparison-select");
    select.innerHTML = teamOptionsHtml;

    select.addEventListener('input', (event) => {addTeamToCompare(event, data)})
    document.querySelector(".comparison-reset").addEventListener('click', function (event) {
        document.querySelector(".comparison-team-cards").innerHTML = "";
    })


}

document.addEventListener("data_loaded", initializeTeamComparisons);