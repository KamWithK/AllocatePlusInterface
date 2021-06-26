<script>
    export let activity
    let startTime = activity.standard_timeblock.start
    let endTime = activity.standard_timeblock.end
</script>

<option activity = {activity}>{startTime}-{endTime}</option>

<style></style>