$(document).ready(function () {
	$('#myTable').DataTable({
	  paging: false,
	  ordering: true,
	  select: {
		items: 'row'
	  },
	  autoWidth: false,
	  responsive: true,
	  fixedColumns:   {
		heightMatch: 'none'
	  },
	  order: [
		[2, "asc"], // free
		[1, "asc"],	// easy
		[3, "asc"], // type
		[0, "asc"]	// title
	  ],
	  searching: false,
	});

	$("#switch").addClass('checked');
	$("#switch").on('click', function() {
		
		if ($(this).hasClass('checked')) {
			document.documentElement.setAttribute('data-bs-theme', 'light');
			//$(this).text("DARK");
			console.log("Switched to light");
		}
		else {
			document.documentElement.setAttribute('data-bs-theme', 'dark');
			//$(this).text("LIGHT");
			console.log("Switched to dark");
		}
		$(this).toggleClass("checked");
	});

	$('#myTable_info').hide();

	console.log('JS done loading');
  });
