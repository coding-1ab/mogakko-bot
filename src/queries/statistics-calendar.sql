select distinct
	date(`joined`, '+09:00') as `date`
from
	`vc_activities`
where
	`user` = ?
