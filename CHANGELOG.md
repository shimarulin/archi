# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 1.6.4 - 2021-10-16


### Bug Fixes

e8323d - move EDITOR and VISUAL environment variables to profile.d files - Vyacheslav Shimarulin


- - -
## 1.6.3 - 2021-09-25


### Bug Fixes

114b7b - update the list of installed packages - Vyacheslav Shimarulin


- - -
## 1.6.2 - 2021-09-18


### Bug Fixes

056a05 - upgrade inquire to 0.2.0 - Vyacheslav Shimarulin


- - -
## 1.6.1 - 2021-08-31


### Bug Fixes

395fbb - upgrade inquire to 0.0.10 - Vyacheslav Shimarulin


- - -
## 1.6.0 - 2021-08-29


### Miscellaneous Chores

b70b0f - upgrade inquire to 0.0.8 - Vyacheslav Shimarulin


### Features

2ce52f - add swap size input - Vyacheslav Shimarulin


### Style

ee2b39 - code style - Vyacheslav Shimarulin


### Bug Fixes

1ae9b4 - question order - Vyacheslav Shimarulin


- - -
## 1.5.1 - 2021-08-22


### Bug Fixes

913ddc - hide password and change summary message styles - Vyacheslav Shimarulin


### Miscellaneous Chores

3bed2b - inquire upgrade - Vyacheslav Shimarulin


- - -
## 1.5.0 - 2021-08-22


### Bug Fixes

fce672 - stack of errors for long error message - Vyacheslav Shimarulin


### Miscellaneous Chores

19588f - welcome message style - Vyacheslav Shimarulin

7078cc - inquire and serge upgrade - Vyacheslav Shimarulin


### Features

8c15b9 - ui controls render config - Vyacheslav Shimarulin

7ce2ce - enable inquire option to reveal password by Ctrl+R - Vyacheslav Shimarulin

40927c - add user help message - Vyacheslav Shimarulin


- - -
## 1.4.1 - 2021-08-19


### Bug Fixes

c74c27 - fix spaces in /etc/profile - Vyacheslav Shimarulin


- - -
## 1.4.0 - 2021-08-19


### Features

d53f71 - update EDITOR, VISUAL and PATH environment variables - Vyacheslav Shimarulin

3ac0d5 - set default text editor (nvim) - Vyacheslav Shimarulin


- - -
## 1.3.0 - 2021-08-17


### Features

8c0597 - hostname validations - Vyacheslav Shimarulin


- - -
## 1.2.0 - 2021-08-17


### Features

11bcda - moved from dialoguer to inquire - Vyacheslav Shimarulin

175955 - replace dialoguer to inquire and add handler to inquire response - Vyacheslav Shimarulin

435bf5 - replace dialoguer::Select to inquire::Select - Vyacheslav Shimarulin

0cfa5b - add post-install message and confirm reboot - Vyacheslav Shimarulin


### Bug Fixes

3b9596 - commit message condition - Vyacheslav Shimarulin

481111 - remove condition - Vyacheslav Shimarulin

c60596 - negative pattern for tags - Vyacheslav Shimarulin

01e5ac - YAML syntax - Vyacheslav Shimarulin

a754e6 - double release - Vyacheslav Shimarulin

0e06f5 - add personal access token - Vyacheslav Shimarulin

0e3fad - only tags on deploy - Vyacheslav Shimarulin

9af5cb - get tags on deploy - Vyacheslav Shimarulin

8aa9db - workflow naming, ignore tags - Vyacheslav Shimarulin

129875 - split release and deploy workflow - Vyacheslav Shimarulin

62ad78 - 'git push' on hook - Vyacheslav Shimarulin

d1b342 - cog settings - Vyacheslav Shimarulin

76a9af - git checkout main - Vyacheslav Shimarulin

4b9ce9 - revert changes from bump version 1.0.1 - Vyacheslav Shimarulin

762636 - push changes to git - Vyacheslav Shimarulin

29a872 - test create release - Vyacheslav Shimarulin


### Miscellaneous Chores

def085 - add banner - Vyacheslav Shimarulin

61a059 - inquire upgrade - Vyacheslav Shimarulin

7ccb93 - add .DS_Store to Git ignore file - Vyacheslav Shimarulin

745adb - 1.1.0 - github-actions

6ecc44 - 1.0.0 - Vyacheslav Shimarulin

e6d4b3 - pre-release changes - Vyacheslav Shimarulin

07cca7 - add license file - Vyacheslav Shimarulin

d96083 - add license info - Vyacheslav Shimarulin

b220d1 - 1.0.11 - github-actions

8f2d6d - 1.0.10 - github-actions

c15025 - 1.0.9 - github-actions

c27265 - 1.0.8 - github-actions

06f64b - 1.0.7 - github-actions

da4935 - 1.0.6 - github-actions

fe0ea5 - 1.0.5 - github-actions

4245d8 - 1.0.4 - github-actions

d34057 - 1.0.3 - github-actions

ca3e29 - 1.0.2 - github-actions

c9b0f1 - 1.0.1 - Vyacheslav Shimarulin

4a5e95 - test on master - Vyacheslav Shimarulin

170efd - 1.0.1 - github-actions

56f6d1 - 1.0.0 - Vyacheslav Shimarulin


### Documentation

5c691f - remove dialoguer note - Vyacheslav Shimarulin

040d11 - use Docker to build with musl - Vyacheslav Shimarulin

707b17 - Setup SSH connection for VM - Vyacheslav Shimarulin

52e8f8 - get archi - Vyacheslav Shimarulin

6deb45 - update features, add usage information and known issues - Vyacheslav Shimarulin


### Tests

d02cbd - event.ref - Vyacheslav Shimarulin


### Refactoring

f31b6b - remove unused config - Vyacheslav Shimarulin

cfd10a - remove unused import - Vyacheslav Shimarulin

6f8269 - rename GitHub action - Vyacheslav Shimarulin


### Continuous Integration

b0d0d6 - remove event.ref - Vyacheslav Shimarulin

e9eb62 - move 'git push' to action step - Vyacheslav Shimarulin

614e49 - update git user - Vyacheslav Shimarulin

a5567e - fix git fetch depth - Vyacheslav Shimarulin

adde9b - fix cog check - Vyacheslav Shimarulin

3c3638 - cog check - Vyacheslav Shimarulin

af14ed - test CWD - Vyacheslav Shimarulin

2e28cc - install workflow dependencies - Vyacheslav Shimarulin

5783c3 - setup workflow - Vyacheslav Shimarulin

9a593b - change default branch name - Vyacheslav Shimarulin

38cb53 - fix string for ignore tags in workflow - Vyacheslav Shimarulin

a86842 - fix ignore tags in workflow - Vyacheslav Shimarulin

9d4e20 - ignore tags in workflow - Vyacheslav Shimarulin

53ef29 - move build to pre-bump hooks - Vyacheslav Shimarulin

e85473 - add Cargo.lock to commit - Vyacheslav Shimarulin

42cf38 - split post-bump command - Vyacheslav Shimarulin

2fee91 - create release - Vyacheslav Shimarulin

e9b4ba - add cocogitto - Vyacheslav Shimarulin

3d5645 - add release action - Vyacheslav Shimarulin


- - -
## 1.1.0 - 2021-06-01


### Documentation

2acf1e - get archi - Vyacheslav Shimarulin


### Features

dba475 - add post-install message and confirm reboot - Vyacheslav Shimarulin


- - -
## 1.0.0 - 2021-05-31


### Documentation

db1f9e - update features, add usage information and known issues - Vyacheslav Shimarulin

2f0443 - description - Vyacheslav Shimarulin

c50933 - principles and features - Vyacheslav Shimarulin

ef921d - ssh connection to virtual host - Vyacheslav Shimarulin

e0f6b3 - workflow notes - Vyacheslav Shimarulin

3dc91d - add build notes to readme - Vyacheslav Shimarulin


### Features

3d5a84 - add git pre-commit hook - Vyacheslav Shimarulin

f1f5ea - set timezone, enable date sync - Vyacheslav Shimarulin

8107e2 - swap off before installation (a disk can be busy) - Vyacheslav Shimarulin

ce2647 - return Output - Vyacheslav Shimarulin

a03802 - rm set root password - Vyacheslav Shimarulin

652a6d - enable Wi-Fi - Vyacheslav Shimarulin

4edbde - add user - Vyacheslav Shimarulin

15f7a2 - enable NetworkManager - Vyacheslav Shimarulin

86c176 - generate grub menu with grub-mkconfig - Vyacheslav Shimarulin

f7c39d - grub menu entry - Vyacheslav Shimarulin

6aaed9 - add grub.cfg to pacman ignore list - Vyacheslav Shimarulin

678ed1 - create file util - Vyacheslav Shimarulin

6f9f12 - change efi directory to /boot/efi - Vyacheslav Shimarulin

3f19bb - set hostname - Vyacheslav Shimarulin

4a922f - set root user password - Vyacheslav Shimarulin

e38a40 - create UEFI menu option - Vyacheslav Shimarulin

5df87f - GRUB2 on UEFI install options update - Vyacheslav Shimarulin

73da12 - GRUB2 on UEFI install successfully - Vyacheslav Shimarulin

faa493 - partitioning and mounting fs for GRUB2 on UEFI - Vyacheslav Shimarulin

a548ac - install GRUB2 to UEFI - Vyacheslav Shimarulin

acbc52 - set hostname - Vyacheslav Shimarulin

c84f11 - grub install - Vyacheslav Shimarulin

1c2d85 - generate fstab - Vyacheslav Shimarulin

bc03a4 - install core packages - Vyacheslav Shimarulin

859f7b - config example - Vyacheslav Shimarulin

950a7c - mount subvolumes - Vyacheslav Shimarulin

c1fbb4 - mount root partition and create subvolume - Vyacheslav Shimarulin

d83d1e - mkswap - Vyacheslav Shimarulin

f8406b - partitioning - Vyacheslav Shimarulin

200d6e - format confirm and exit messages - Vyacheslav Shimarulin

05d9ea - exit if user not confirm the installation - Vyacheslav Shimarulin

b19673 - confirm installation - Vyacheslav Shimarulin

c82aa3 - select disk prompt struct - Vyacheslav Shimarulin

8a6d36 - select disk prompt theme extra - Vyacheslav Shimarulin

c9fae4 - prompt theme - Vyacheslav Shimarulin

1e85a8 - disk path - Vyacheslav Shimarulin

48a676 - theme and format for disk select - Vyacheslav Shimarulin

9c5b02 - create disk select - Vyacheslav Shimarulin

236e3e - detect disk devices - Vyacheslav Shimarulin

35ca38 - get facts about firmware interface - Vyacheslav Shimarulin

3e9736 - ask username and password - Vyacheslav Shimarulin


### Tests

f3a21b - event.ref - Vyacheslav Shimarulin


### Miscellaneous Chores

656d41 - pre-release changes - Vyacheslav Shimarulin

cf71ce - add license file - Vyacheslav Shimarulin

a386ab - add license info - Vyacheslav Shimarulin

de2826 - 1.0.11 - github-actions

0f6607 - 1.0.10 - github-actions

0c7601 - 1.0.9 - github-actions

2f9764 - 1.0.8 - github-actions

f76967 - 1.0.7 - github-actions

69e6fb - 1.0.6 - github-actions

ffa51d - 1.0.5 - github-actions

ed5207 - 1.0.4 - github-actions

fa1631 - 1.0.3 - github-actions

503c0d - 1.0.2 - github-actions

9684a9 - 1.0.1 - Vyacheslav Shimarulin

39df83 - test on master - Vyacheslav Shimarulin

f838bf - 1.0.1 - github-actions

5b18bc - 1.0.0 - Vyacheslav Shimarulin

82fcf9 - confirm message color - Vyacheslav Shimarulin


### Continuous Integration

616930 - remove event.ref - Vyacheslav Shimarulin

e352c1 - move 'git push' to action step - Vyacheslav Shimarulin

34cc89 - update git user - Vyacheslav Shimarulin

316a16 - fix git fetch depth - Vyacheslav Shimarulin

431e5b - fix cog check - Vyacheslav Shimarulin

05b37a - cog check - Vyacheslav Shimarulin

3e555a - test CWD - Vyacheslav Shimarulin

54f5fa - install workflow dependencies - Vyacheslav Shimarulin

77463f - setup workflow - Vyacheslav Shimarulin

2fc100 - change default branch name - Vyacheslav Shimarulin

5ca7cc - fix string for ignore tags in workflow - Vyacheslav Shimarulin

02d293 - fix ignore tags in workflow - Vyacheslav Shimarulin

24948e - ignore tags in workflow - Vyacheslav Shimarulin

0426e0 - move build to pre-bump hooks - Vyacheslav Shimarulin

7089b6 - add Cargo.lock to commit - Vyacheslav Shimarulin

93b9d6 - split post-bump command - Vyacheslav Shimarulin

3d1979 - create release - Vyacheslav Shimarulin

536caa - add cocogitto - Vyacheslav Shimarulin

836339 - add release action - Vyacheslav Shimarulin


### Style

8b5891 - fmt - Vyacheslav Shimarulin

062d63 - select disk prompt struct - Vyacheslav Shimarulin

28a234 - reformat project with cargo - Vyacheslav Shimarulin


### Bug Fixes

a5c346 - commit message condition - Vyacheslav Shimarulin

8840e4 - remove condition - Vyacheslav Shimarulin

e58c0d - negative pattern for tags - Vyacheslav Shimarulin

a90eb8 - YAML syntax - Vyacheslav Shimarulin

3f26cf - double release - Vyacheslav Shimarulin

4a1c3c - add personal access token - Vyacheslav Shimarulin

659e00 - only tags on deploy - Vyacheslav Shimarulin

9010b7 - get tags on deploy - Vyacheslav Shimarulin

519831 - workflow naming, ignore tags - Vyacheslav Shimarulin

f7212c - split release and deploy workflow - Vyacheslav Shimarulin

b70f17 - 'git push' on hook - Vyacheslav Shimarulin

3d9afc - cog settings - Vyacheslav Shimarulin

d40694 - git checkout main - Vyacheslav Shimarulin

7ae4c1 - revert changes from bump version 1.0.1 - Vyacheslav Shimarulin

00e932 - push changes to git - Vyacheslav Shimarulin

b664ea - test create release - Vyacheslav Shimarulin

5ab139 - command name - Vyacheslav Shimarulin

9f51b9 - file path - Vyacheslav Shimarulin

5739cc - write file - Vyacheslav Shimarulin

2f00f3 - hostnamectl output status - Vyacheslav Shimarulin

35ae65 - UEFI boot option - Vyacheslav Shimarulin

832215 - hostname prompt - Vyacheslav Shimarulin


### Refactoring

5e2f81 - rename GitHub action - Vyacheslav Shimarulin

0a7f44 - swap utils - Vyacheslav Shimarulin

d9b51f - packages - Vyacheslav Shimarulin

59a1cf - bootloader - Vyacheslav Shimarulin

8900a5 - disk operations - Vyacheslav Shimarulin

32e63c - generate fstab - Vyacheslav Shimarulin

6195b4 - use file utils to update hosts file - Vyacheslav Shimarulin

365b94 - use file::create to hostname - Vyacheslav Shimarulin

5f031d - comment unused code - Vyacheslav Shimarulin

fe1e5e - disk and swap modules - Vyacheslav Shimarulin

9ee8d5 - rename mods - Vyacheslav Shimarulin

0ea385 - add config struct - Vyacheslav Shimarulin

18f393 - rename questions to ask_questions - Vyacheslav Shimarulin


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).