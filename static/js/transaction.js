// load list of transactions
let transaction_list = document.querySelector("#transaction_list");
fetch("/api/transaction")
.then((response) => response.json())
.then((data) => {
	let dom_container = document.createElement("div");
	dom_container.classList.add("container");
	let dom_row = document.createElement("div");
	dom_row.classList.add("row");
	let dom_column_base = document.createElement("div");
	dom_column_base.classList.add("d-flex");
	dom_column_base.classList.add("align-items-center");
	let dom_column = dom_column_base.cloneNode();
	dom_column.classList.add("col");
	let dom_column_auto = dom_column_base.cloneNode();
	dom_column_auto.classList.add("col-auto");

	for (item of data) {
		let amount_value = item["amount"];
		let datetime_data = item["datetime"] * 1000;
		datetime_data = new Date(datetime_data);
		let datetime_value;
		{
			let date = `${datetime_data.getDate()}`.padStart(2, '0');
			let month = `${datetime_data.getMonth() + 1}`.padStart(2, '0');
			let year = datetime_data.getFullYear();
			let hour = `${datetime_data.getHours()}`.padStart(2, '0');
			let minute = `${datetime_data.getMinutes()}`.padStart(2, '0');
			let second = `${datetime_data.getSeconds()}`.padStart(2, '0');
			datetime_value = `${date}/${month}/${year} ${hour}:${minute}:${second}`
		}
		let li = document.createElement('li');
		li.classList.add("list-group-item");
		let container1 = dom_container.cloneNode();
		let row = dom_row.cloneNode();

		let column1 = dom_column_auto.cloneNode();
		let icon_svg = document.createElement("svg");
		icon_svg.classList.add("bi");
		icon_svg.classList.add("bi-1-circle");
		column1.appendChild(icon_svg);
		row.appendChild(column1);

		let column2 = dom_column.cloneNode();
		row.appendChild(column2);

		let column3 = dom_column.cloneNode();
		let column3container = dom_container.cloneNode();
		let column3row1 = dom_row.cloneNode();
		let column3row2 = dom_row.cloneNode();
		let amount_span = document.createElement("span");
		let amount_text = document.createTextNode(amount_value);
		let datetime_span = document.createElement("span");
		let datetime_text = document.createTextNode(datetime_value);
		amount_span.appendChild(amount_text);
		datetime_span.appendChild(datetime_text);
		column3row1.appendChild(amount_span);
		column3row2.appendChild(datetime_span);
		column3container.appendChild(column3row1);
		column3container.appendChild(column3row2);
		column3.appendChild(column3container);
		row.appendChild(column3);

		let column4 = dom_column_auto.cloneNode();
		row.appendChild(column4);
		let edit_button = document.createElement("button");
		edit_button.classList.add("btn");
		edit_button.classList.add("btn-sm");
		edit_button.classList.add("btn-primary");
		column4.appendChild(edit_button);
		let pencil_svg = document.createElement("svg");
		pencil_svg.classList.add("bi");
		pencil_svg.classList.add("bi-pencil-square");
		edit_button.appendChild(pencil_svg);

		li.appendChild(container1);
		container1.appendChild(row);
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
	now_date += `${String(now.getMonth()+1).padStart(2, '0')}-`;
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
