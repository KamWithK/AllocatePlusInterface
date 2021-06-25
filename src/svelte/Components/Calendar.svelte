<script>
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let times = ["7am","","8am", "", "9am", "", "10am", "", "11am", "", "12pm", "", "1pm", "", "2pm", "", "3pm", "", "4pm", "", "5pm", "", "6pm", "", "7pm", "", "8pm", "", 
    "9pm", ""]
    let collisions = [[0,0,0,0,0,0,1,0,0,0,1,0,0,0,2,0,2,0,2,0,2,0,2,0,0,0,0,0,0,1],[0,0,2,0,0,0,2,0,1,0,1,0,0,0,2,0,1,0,2,0,1,0,1,0,0,0,0,0,0,0],[0,0,1,0,0,0,1,0,0,0,3,0,0,0,0,0,1,0,3,0,2,0,4,0,0,0,0,0,0,0],[0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,2,0,1,0,1,0,1,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]

    function buildTable(collisions){
        var table = document.getElementById("myTable")
        var noRows = table.rows.length
        var noCols = table.rows[0].cells.length

        for (var x= 1; x < noCols; x++) {
            for (var y = 1; y < noRows; y++){
                table.rows[y].cells[x].style.backgroundColor = 'rgba(' + 15 + ',' + 255 + ',' + 255 + ',' + generateOpacity(collisions[x-1][y-1]) + ')';
            }
        }
    }

    function generateOpacity(collisionCell){
        var opacity = 0;
        let collisionCellRange = [...Array(collisionCell).keys()]
        for (const _ in collisionCellRange){
            if (opacity <= 1){
                opacity += 0.2
            } else {
                break;
            }
        }
        return opacity;
    }
</script>

<table id = "myTable">
    <!-- Heading for days -->
    <tr>
        <!-- Empty cell for times column -->
        <th class = "times"></th>
        {#each days as day}
            <th>{day}</th>
        {/each}
    </tr>
        <!-- Heading for times -->
    {#each times as time}
        <tr>
            <th class = "times">{time}</th>
            {#each days as day}
                <td class = "times"></td>
            {/each}
        </tr>
    {/each}
</table>

<button on:click={() => buildTable(collisions)}>Build Table</button>

<style>
    table{
        table-layout:fixed;
        border-collapse: separate;
        border-spacing: 0;
        width: 100%;
        background: #ffffff;
        box-shadow: 9.91px 9.91px 15px #E8E8E8, -9.91px -9.91px 15px #FFFFFF;
    }

    .times{
        width: 50px;
        height: 20px;
        text-align: center;
    }

    :nth-child(odd) td{
        border: 1px solid white;
    }

    tr, th, td {
        border: 1px solid rgba(161, 161, 161, 0.3);
        padding: 3px;
    }

    th {
        font-weight: normal;
    }

    
</style>