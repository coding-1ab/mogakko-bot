with
`lb` as (
	select
		`user`,
		coalesce(
			sum(
				unixepoch(coalesce(`left`, 'now'))
				- unixepoch(`joined`)
			),
		0) as `total_duration`
	from
		`vc_activities`
	group by
		`user`
),
`rlb` as (
	select
		*,
		rank() over (
			order by `total_duration` desc
		) as `rank`
	from
		`lb`
)
select
	`rank`,
	`user`,
	`total_duration`
from
	`rlb`
where
	`user` = ?

