// TODO: Add Search Feature
// TODO: Team Comparison Feature
// TODO: Dot menu for teams to access more info

function round2Two(number) {
  return +(Math.round(number + "e+2")  + "e-2");
}

function toggleCardOptions(event) {
  let teamCard = event.target.parentElement;
  let optionsMenu = teamCard.querySelector(".card-options-menu");
  console.log(event);
  if (optionsMenu.classList.contains("visible")) {
    console.log("visible")
    optionsMenu.classList.remove("visible")
  } else {
    console.log("not visible")
    optionsMenu.classList.add("visible");
  }
}

function fillRawTeamData(event) {
    let data = event.detail.scout_data;
    let teams = Object.values(data);
    
    let teamEntryList = document.querySelector(".data-team-raw-display");
    teams.forEach(team => {
      let teamEntry = document.createElement("div");
      teamEntry.classList.add("team-entry")
      // May be unwise to use innerHtml as it will execute any html put in the variables...but its probably fine...

      teamEntry.innerHTML = `
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
      `
      teamEntryList.appendChild(teamEntry);

      

    });
    Array.from(teamEntryList.getElementsByClassName("dot-menu")).forEach(menu => {
      menu.addEventListener("click", toggleCardOptions);
      console.log("added event listener");
    })
}
  
document.addEventListener("data_loaded", fillRawTeamData);
  