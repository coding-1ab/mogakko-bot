select
	count(*) as `count`
from
	`vc_activities`
where
	`user` = ?
	and date(`joined`, '+09:00') = date('now', '+09:00')
	and `left` is not null
