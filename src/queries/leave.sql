update
	`vc_activities`
set
	`left` = datetime('now')
where
	`id` = ?
