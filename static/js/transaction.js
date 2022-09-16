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
// DOM transaction_list
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


// datetime mode
let datetime_mode = document.querySelector("#datetime_mode");
let datetime_modes = document.querySelector("#datetime_modes");
let datetime_now_button = document.querySelector("#datetime_now_button");
let datetime_custom_button = document.querySelector("#datetime_custom_button");

let current_datetime_mode = datetime_mode.textContent;

let update_datetime_timeout;
function update_datetime_input(once) {
	if (current_datetime_mode != "Now") {
		return;
	}
	let now = new Date();
	let now_date = "";
	now_date += `${String(now.getFullYear()).padStart(2, '0')}-`;
	now_date += `${String(now.getMonth()).padStart(2, '0')}-`;
	now_date += `${String(now.getDate()).padStart(2, '0')}`;
	let now_time = "";
	now_time += `${String(now.getHours()).padStart(2, '0')}:`;
	now_time += `${String(now.getMinutes()).padStart(2, '0')}`;
	date_input.value = now_date;
	time_input.value = now_time;
	set_datetime_value(datetime_input, date_input, time_input);
	if (once) {
		return;
	}
	start_update_datetime_input();
}

function start_update_datetime_input() {
	update_datetime_input(true);
	update_datetime_timeout = setTimeout(
		update_datetime_input,
		1000, // Milliseconds
	)
}

function stop_update_datetime_input() {
	clearTimeout(update_datetime_timeout);
}

if (current_datetime_mode == "Now") {
	start_update_datetime_input();
}

datetime_now_button.addEventListener("click", () => {
	datetime_mode.textContent = "Now";
	datetime_custom_button.classList.remove("active");
	datetime_now_button.classList.add("active");
	start_update_datetime_input();
});
datetime_custom_button.addEventListener("click", () => {
	datetime_mode.textContent = "Custom";
	datetime_now_button.classList.remove("active");
	datetime_custom_button.classList.add("active");
	date_input.value = "";
	time_input.value = "";
	stop_update_datetime_input();
});
