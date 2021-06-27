<script>
    export let activity

    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    let start = new Date(activity.standard_timeblock.start)
    let end = new Date(activity.standard_timeblock.end)

    let day = days[start.getDay()]
    let start_time = start.getHours() + ":" + start.getMinutes()
    let end_time = end.getHours() + ":" + end.getMinutes()
</script>

<option activity={activity}>
    {day} {start_time}-{end_time}
</option>

<style></style>