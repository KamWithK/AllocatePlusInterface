<script>
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let times = ["8:00", "8:30", "9:00", "9:30", "10:00", "10:30", "11:00", "11:30", "12:00", "12:30", "1:00", "1:30", "2:00", "2:30", "3:00", "3:30", "4:00", "4:30", "5:00", "5:30", "6:00", "6:30", "7:00"]
</script>

<table style="width:100%">
    <!-- Heading for days -->
    <tr>
        <!-- Empty cell -->
        <th></th>
        {#each days as day}
            <th>{day}</th>
        {/each}
    </tr>
    
    <!-- Heading for times -->
    {#each times as time}
      <tr class = "times">
          <th>{time}</th>
      </tr>
    {/each}
</table>

<style>
    table{
        table-layout: fixed;
        background: #d8fcfd;
        box-shadow: 0.3px 0.3px 15px #C5E5E6, -0.3px -0.3px 15px #EBFFFF;
    }
</style>