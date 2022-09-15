// load list of transactions
let transaction_list = document.querySelector("#transaction_list");
fetch("/api/transaction")
	.then((response) => response.json())
	.then((data) => {
		for (item of data) {
			let li = document.createElement('li');
			li.classList.add("list-group-item");
			// let liNode = document.createTextNode(item["amount"]);
			// li.appendChild(liNode);
			let row = document.createElement("div");
			row.classList.add("row");
			li.appendChild(row);
			let column1 = document.createElement("div");
			column1.classList.add("col");
			column1.classList.add("d-flex");
			column1.classList.add("align-items-center");
			row.appendChild(column1);
			let amount_text = document.createTextNode(item["amount"]);
			column1.appendChild(amount_text);
			let column2 = document.createElement("div");
			column2.classList.add("col-auto");
			column2.classList.add("d-flex");
			column2.classList.add("align-items-center");
			row.appendChild(column2);
			let edit_button = document.createElement("button");
			edit_button.classList.add("btn");
			edit_button.classList.add("btn-sm");
			edit_button.classList.add("btn-primary");
			column2.appendChild(edit_button);
			let pencil_svg = document.createElement("svg");
			pencil_svg.classList.add("bi");
			pencil_svg.classList.add("bi-pencil-square");
			edit_button.appendChild(pencil_svg);
			transaction_list.appendChild(li);
		}
	});


// edit transaction
transaction_list.addEventListener("dblclick", function(click) {
	console.log(click);
});


// datetime
let datetime_input = document.querySelector("#datetime_input");
let date_input = document.querySelector("#date_input");
let time_input = document.querySelector("#time_input");

function datetime_value(
	date_string = (new Date()).toDateString(),
	time_string = ""
) {
	return Math.round(
		(new Date(
			`${date_string} ${time_string}`.trim()
		))
		.getTime()/1000 // JS UTC is in milliseconds
	)
}

function set_datetime_value(datetime_input, date_input, time_input) {
	datetime_input.value = datetime_value(date_input.value, time_input.value);
}

date_input.value = "";
time_input.value = "";
set_datetime_value(datetime_input, date_input, time_input);
date_input.addEventListener("change",
	() => set_datetime_value(datetime_input, date_input, time_input));
time_input.addEventListener("change",
	() => set_datetime_value(datetime_input, date_input, time_input));
