<script>
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let times = ["7:00","7:30","8:00", "8:30", "9:00", "9:30", "10:00", "10:30", "11:00", "11:30", "12:00", "12:30", "1:00", "1:30", "2:00", "2:30", "3:00", "3:30", "4:00", "4:30", "5:00", "5:30", "6:00", "6:30", "7:00", "7:30", "8:00", "8:30", 
    "9:00", "9:30", "10:00"]
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
        </tr>
    {/each}
</table>

<style>
    table{
        table-layout:fixed;
        width: 100%;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }

    .times{
        width: 10%;
        text-align: center;
    }

    tr, th {
        border: black solid;
    }

    
</style>