

function getValueStroke(value) {
  if (value < 30) return "red";
  else if (value < 60) return "orange";
  else if (value < 90) return "lightgreen";
  else return "limegreen";
}

function updateTeamCard(event, data) {
    let teamCard = document.getElementById(event.srcElement.dataset.cardId);
    let teamNumber = event.srcElement.value;
    if (!teamNumber) return;
    let team = data[teamNumber];
    // TODO: Implement Charge Station Stats
    // TODO: Add Progress/Circle Percentage Meters for Charge Station Stats
    // TODO: Add Export to Pretty Image/Banner of the Match
    
    teamCard.innerHTML = `
    <div class="label">${team.team_number}</div>
                  <div class="team-name">${(team.tba_data) ? team.tba_data.nickname:"Team"} (${team.match_data.length} ${(team.match_data.length == 1) ? "Entry":"Entries" })</div>
          
                  <div class="cubes-row">
                
                    <table class="cube-table">
                      <tr>
                        <th>Low</th>
                        <th>Medium</th>
                        <th>High</th>
                      </tr>
                      <tr>
                      <td>${round2Two(team.summary.avg_low)}<span class='confidence-interval'>±${round2Two(team.summary.low_confidence)}</span></td>
                      <td>${round2Two(team.summary.avg_med)}<span class='confidence-interval'>±${round2Two(team.summary.med_confidence)}</span></td>
                      <td>${round2Two(team.summary.avg_high)}<span class='confidence-interval'>±${round2Two(team.summary.high_confidence)}</span></td>
                      </tr>
                    </table>
                  </div>
                  <table class="station-table">
                  <tr>
                    <th class="station-text">Smart Links</th>
                    <th class="station-text">Teleop Points</th>
                    <th class="station-text">Auto Points</th>
                  </tr>
                  <tr>
                    <td style="text-align:center;">${round2Two(team.summary.avg_links)}</td>
                    <td style="text-align:center;">${round2Two(team.summary.teleop_points)}<span class='confidence-interval'>±${round2Two(team.summary.teleop_point_confidence)}</span></td>
                    <td style="text-align:center;">${round2Two(team.summary.auto_points)}<span class='confidence-interval'>±${round2Two(team.summary.auto_point_confidence)}</span></td>
                  </tr>
                </table>
              <table class="station-table">
                <tr>
  
                  <th class="station-text">Dock %</th>
                  <th></th>
                  <th class="station-text">Engage %</th>
                </tr>
                <tr>
    
                  <td><div class="percentage-bar ldBar label-center"></div></td>
                  <td></td>
                  <td><div class="percentage-bar ldBar label-center"></div></td>
                </tr>
              </table>
              `
              
    let percentageBars = teamCard.getElementsByClassName("percentage-bar");
    new ldBar(percentageBars[0], { "value": Math.round(team.summary.balance_percentage*100), "stroke": "white", "preset": "circle"});
    new ldBar(percentageBars[1], { "value": Math.round(team.summary.dock_percentage*100), "stroke": "white", "preset": "circle"});

 }

function initializeMatchOverview(event) {
    let data = event.detail.scout_data;
    let teamNumbers = Object.keys(data);

    let teamOptionsHtml = "<option value='' selected disabled hidden>Team</option>"
    teamNumbers.forEach(teamNumber => {
        
        
        teamOptionsHtml += `<option value=${teamNumber}>${teamNumber}</option>`
        
        
    })

    let teamSelects = document.getElementsByClassName("team-select");
    for (const select of teamSelects) {
        select.innerHTML = teamOptionsHtml;
        select.addEventListener("input", (event) => {updateTeamCard(event, data)});
    }

}

document.addEventListener("data_loaded", initializeMatchOverview);