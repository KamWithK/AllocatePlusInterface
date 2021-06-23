<script>
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let times = ["7am","","8am", "", "9am", "", "10am", "", "11am", "", "12pm", "", "1pm", "", "2pm", "", "3pm", "", "4pm", "", "5pm", "", "6pm", "", "7pm", "", "8pm", "", 
    "9pm", "", "10pm"]
</script>

<table>
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
                <td></td>
            {/each}
        </tr>
    {/each}
</table>

<style>
    table{
        table-layout:fixed;
        border-collapse: collapse;
        width: 100%;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }

    .times{
        width: 50px;
        height: 20px;
        text-align: center;
    }

    tr, th, td {
        border: solid black;
    }

    
</style>