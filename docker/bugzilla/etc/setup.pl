# https://www.bugzilla.org/docs/3.0/html/api/checksetup.html#RUNNING_CHECKSETUP_NON-INTERACTIVELY

$answer{'db_driver'} = 'sqlite';
$answer{'db_name'} = 'bugs';
# pwgen 128 1
$answer{'site_wide_secret'} = 'bla-bla-bla';
$answer{'urlbase'} = 'https://bugs.my-domain.org/';

$answer{'ADMIN_EMAIL'} = 'who-am-i@gmail.com';
$answer{'ADMIN_PASSWORD'} = 'change-me';
$answer{'ADMIN_REALNAME'} = 'Web Master';

# smtp.qq.com
$answer{'SMTP_SERVER'} = 'smtp.gmail.com';

# $answer{'NO_PAUSE'} = 1
