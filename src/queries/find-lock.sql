select
	`id`
from
	`vc_activities`
where
	`user` = ?
	and `left` is null
