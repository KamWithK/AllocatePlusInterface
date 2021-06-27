<script>
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let times = ["7am","","8am", "", "9am", "", "10am", "", "11am", "", "12pm", "", "1pm", "", "2pm", "", "3pm", "", "4pm", "", "5pm", "", "6pm", "", "7pm", "", "8pm", "", 
    "9pm", ""]
    let collisions = [[0,0,0,0,0,0,1,1,1,1,1,1,1,1,3,3,4,4,4,4,6,6,3,3,3,3,0,0,0,0],[0,0,3,3,3,3,4,4,5,5,4,4,3,3,3,3,5,5,5,5,6,6,3,3,2,2,0,0,0,0],[0,0,3,3,3,3,2,2,2,2,4,4,4,4,2,2,4,4,5,5,7,7,6,6,6,6,0,0,0,0],[0,0,2,2,2,2,3,3,3,3,3,3,3,3,1,1,3,3,3,3,5,5,5,5,3,3,1,1,0,0],[0,0,1,1,3,3,2,2,3,3,4,4,3,3,4,4,4,4,2,2,2,2,0,0,0,0,0,0,0,0]]

    function buildTable(collisions){
        var table = document.getElementById("calendar")
        var noRows = table.rows.length
        var noCols = table.rows[0].cells.length

        for (var x= 1; x < noCols; x++) {
            for (var y = 1; y < noRows; y++){
                table.rows[y].cells[x].style.backgroundColor = "rgba(" + 20 + "," + 70 + "," + 200 + "," + generateOpacity(collisions[x-1][y-1]) + ")";
            }
        }
    }

    function generateOpacity(collision_cell){
        var opacity = 0;
        let collision_cell_range = [...Array(collision_cell).keys()]
        for (const _ in collision_cell_range){
            if (opacity <= 1){
                opacity += 0.2
            } else {
                break;
            }
        }

        return opacity;
    }
</script>

<table id="calendar">
    <!-- Heading for days -->
    <tr>
        <!-- Empty cell for times column -->
        <th width=50px class="times text-base font-normal"></th>
        {#each days as day}
            <th class="border">{day}</th>
        {/each}
    </tr>

    <!-- Heading for times -->
    {#each times as time}
        <tr>
            <th class="border text-base font-normal">{time}</th>
            {#each days as _}
                <td class="border">&#8193‎</td>
            {/each}
        </tr>
    {/each}
</table>

<button on:click={() => buildTable(collisions)} class="m-2">Build Table</button>

<style>
</style>