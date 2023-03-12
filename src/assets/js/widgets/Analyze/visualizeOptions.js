function round2Two(number) {
    return +(Math.round(number + "e+2")  + "e-2");
}

function resetRankings(data) {
    document.querySelector(".rankings").innerHTML = "";
}

function getValueStroke(value) {
    if (value < 30) return "red";
    else if (value < 60) return "orange";
    else if (value < 90) return "lightgreen";
    else return "limegreen";
}

function populate_rankings(data, options) {
    resetRankings(data);
    if (options["comparison_team"] == "none") options["comparison_team"] = null;
    console.log(data);
    invoke('get_team_rankings', {'teamData':data, 'options':options}).then((team_rankings) => {
        let rankings = document.querySelector(".rankings");


        for (let i = 0; i < team_rankings.length; i++) {
            let team = team_rankings[i];
            div = document.createElement("div");
            div.classList.add("analysis-rank-entry");
            div.innerHTML = `
            <div class="id-data">
                <div class="rank">#${i+1}</div>
                <div class="team-number">${team.team_number}</div>
            </div>  
            <div class="rate-data">
                  <div class="rating-group">
                    <div class="rating-label">Low</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                    </div>
                <div class="rating-group">
                    <div class="rating-label">Medium</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group" style="margin-right:10px">
                    <div class="rating-label">High</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group">
                    <div class="rating-label">Balance</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                <div class="rating-group" style="margin-right:20px">
                    <div class="rating-label">Dock</div>
                    <div class="sub-rating rating-bar ldBar label-center"></div>
                </div>
                  <div class="overall-rating rating-bar ldBar label-center"></div>
                </div>
              `;

            let ratings = div.getElementsByClassName("rating-bar");
            function makeRating(element, rating) {
                let display_rating = Math.round(rating*100);
                new ldBar(element, { "value": display_rating, "stroke": getValueStroke(display_rating), "preset": "circle"});
            }

            let piece_rating_average = Math.round(((team.low_rating+team.medium_rating+team.high_rating)/3)*100);
            makeRating(ratings[0], team.low_rating);
            makeRating(ratings[1], team.medium_rating);
            makeRating(ratings[2], team.high_rating);
            makeRating(ratings[3], team.balance_rating);
            makeRating(ratings[4], team.dock_rating);
            makeRating(ratings[5], team.overall_rating);
            rankings.appendChild(div);

        }
    }, (reason) => { console.error(reason);});
}

function populateOptions(data) {
    let team_keys = Object.keys(data);

    let teamOptionsHtml = "<option value='none' selected>Individual</option>"
    team_keys.forEach(teamNumber => {
        teamOptionsHtml += `<option value=${teamNumber}>${teamNumber}</option>`
    });
    // Adds an event listener to the team select dropdown
    document.querySelector(".comparison-team-select").addEventListener("change", (event) => {
        let teamNumber = event.target.value;
        let team = data[teamNumber];
        populate_rankings(data, {"comparison_team": team})
    });
    document.querySelector(".comparison-team-select").innerHTML = teamOptionsHtml;
    
    populate_rankings(data, {});

    
}

function initialize(event) {
    let scout_data = event.detail.scout_data;
    populateOptions(scout_data);
}

document.addEventListener("data_loaded", initialize)

