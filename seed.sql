INSERT INTO projects (name, repository_url, color, description, slug)
VALUES ('aletheia',
        'http://github.com/hacknyu/aletheia',
        '406e8e',
        'The single source of truth for HackNYU''s services',
	'aletheia'),
	('styx',
	 'http://github.com/hacknyu/styx',
	 'cbf7ed',
	 'The main entrypoint for hackers',
	 'styx');

INSERT INTO users (display_name, email, password_digest)
VALUES ('nicholaslyang',
        'nick@nicholasyang.com',
	'mypasswordblahblahblah');



INSERT INTO submissions (user_id, project_id)
(SELECT users.id, projects.id FROM users, projects);

