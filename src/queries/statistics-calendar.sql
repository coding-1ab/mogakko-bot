select distinct
	date(`joined`, '+09:00') as `days`
from
	`vc_activities`
where
	`user` = ?
