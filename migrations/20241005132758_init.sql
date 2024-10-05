-- Add migration script here
create table if not exists vc_activities (
	`id` integer primary key,
	`user` text not null,
	`joined` datetime not null default current_timestamp,
	`left` datetime
);

