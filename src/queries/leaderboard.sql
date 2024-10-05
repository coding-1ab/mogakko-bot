select
	`user`,
	count(
		distinct date(`joined`, '+09:00')
	) as days,
	coalesce(
		sum(unixepoch(coalesce(`left`, 'now')) - unixepoch(`joined`)),
	0) as `total_duration`
from
	`vc_activities`
group by
	`user`
order by
	`total_duration` desc
limit 5
