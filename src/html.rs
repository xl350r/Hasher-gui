//html.rs
pub struct Html;
impl Html {
	pub fn content() -> String {
		let html = format!("{}", r#"
<!doctype html>
<html>
<head>
<meta name="viewport" content="text/html; width=device-width, initial-scale=1" http-equiv="content-type">
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
<style>
	.tooltip {
	  position: relative;
	  display: inline-block;
	  border-bottom: 1px dotted black;
	}
	.tooltip .tooltiptext {
	  visibility: hidden;
	  width: 150px;
	  background-color: black;
	  color: White;
	  text-align: center;
	  border-radius: 6px;
	  padding: 5px 0;

	  /* Position the tooltip */
	  position: absolute;
	  z-index: 1;
	}
	.tooltip .tooltiptext::after {
	  content: "";
	  position: absolute;
	  bottom: 100%;
	  left: 50%;
	  margin-left: -5px;
	  border-width: 5px;
	  border-style: solid;
	  border-color: transparent transparent black transparent;
	}
	.tooltip:hover .tooltiptext {
	  visibility: visible;
	}
	body {
		background-color: gray;
	}
	table {
	  border-collapse: collapse;
	  width: 100%;
	}

	th, td {
	  text-align: left;
	  padding: 8px;
	}
	tr:nth-child(odd)  {background-color: white}
	tr:nth-child(even) {background-color: #c7ebff;}
		body {
		  font-family: Arial, Helvetica, sans-serif;
		}

		.navbar {
		  overflow: hidden;
		  background-color: #333;
		}

		.navbar a {
		  float: left;
		  font-size: 16px;
		  color: white;
		  text-align: center;
		  padding: 14px 16px;
		  text-decoration: none;
		}

		.dropdown {
		  float: left;
		  overflow: hidden;
		}

		.dropdown .dropbtn {
		  font-size: 16px;  
		  border: none;
		  outline: none;
		  color: white;
		  padding: 14px 16px;
		  background-color: #333;
		  font-family: inherit;
		  margin: 0;
		}

		.navbar a:hover, .dropdown:hover .dropbtn {
		  background-color: red;
		}

		.dropdown-content {
		  display: none;
		  position: absolute;
		  background-color: #f9f9f9;
		  min-width: 160px;
		  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
		  z-index: 1;
		}

		.dropdown-content a {
		  float: none;
		  color: black;
		  padding: 12px 16px;
		  text-decoration: none;
		  display: block;
		  text-align: left;
		}

		.dropdown-content a:hover {
		  background-color: #ddd;
		}

		.dropdown:hover .dropdown-content {
		  display: block;
		}
		</style>
</head>
    <body>
        <div class="navbar"> 
            <div class="dropdown">
                <button class="dropbtn">Open
                    <i class="fa fa-caret-down"> </i>
                </button> 
                <div class="dropdown-content">
                    <a href='#' onclick="external.invoke('openf')">File</a>
                    <a href='#' onclick="external.invoke('opend')">Directory</a>
                </div>
            </div>
                <div class="tooltip"><a href='#' onclick="external.invoke('save')">Save</a> <span class="tooltiptext"> Save table as JSON to file.</span> </div>
                <div class="tooltip"><a href='#' onclick="external.invoke('load')">load</a> <span class="tooltiptext"> Load from file.</span></div>
                <div class="tooltip"><a href='#' onclick="external.invoke('copy')">Copy</a> <span class="tooltiptext">Copy table as json to clipboard.</span></div>
                <div class="tooltip"><a href='#' onclick="external.invoke('exit')">Exit</a> <span class="tooltiptext">Close Program.</span></div>
        </div>
        <div style="overflow-x:auto;">
        <font size="1">
        <table id="hashtable"></table>
		</font>
		</div>
		<script type="text/javascript">
		function deleteRow(btn) {
			var row = btn.parentNode.parentNode;
			row.parentNode.removeChild(row);
		}
		function generateTableHead(table, data) {
			var thead = table.createTHead();
			var row =  thead.insertRow();
			/* var th = document.createElement("th");
			var text = document.createTextNode("delete")
			th.appendChild(text);
			row.appendChild(th); */ 
			for (k=0 ; k < data.length; k++) {
				var th = document.createElement("th");
				var text = document.createTextNode(data[k]);
				th.appendChild(text);
				row.appendChild(th);
			}
		};
		function generateTable(table, data) {
			for (element=0; element< data.length ; element++) {
				var row = table.insertRow();
				/* var cell = row.insertCell();
				cell.innerHTML = "<input type='button' value='Delete' onclick='deleteRow(this)'>" */
				var file = document.createTextNode(data[element].File);
				var md5 = document.createTextNode(data[element].Md5);
				var whirl = document.createTextNode(data[element].Whirlpool);
				var sha256 = document.createTextNode(data[element].Sha256);
				var sha512 = document.createTextNode(data[element].Sha512);
				var entries = [file, md5, whirl, sha256, sha512]
				for(i=0; i< entries.length; i++ ) {
					var cell = row.insertCell();
					cell.appendChild(entries[i])
				}
				/*
				var cell = row.insertCell();
				cell.appendChild(file);
				var cell = row.insertCell();
				cell.appendChild(md5);
				var cell = row.insertCell();
				cell.appendChild(whirl);
				var cell = row.insertCell();
				cell.appendChild(sha256);
				var cell = row.insertCell();
				cell.appendChild(sha512);
				*/
			}
		};
		let data = Object.keys({ File: "", Md5: "" , Whirlpool: "", Sha256: "", Sha512: ""})
		generateTableHead(document.querySelector('#hashtable'), data)
		</script>
    </body>
</html>"#
);
		html
	}
}