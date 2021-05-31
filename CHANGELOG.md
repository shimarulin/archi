# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 1.0.9 - 2021-05-31


### Bug Fixes

e58c0d - negative pattern for tags - Vyacheslav Shimarulin


- - -
## 1.0.8 - 2021-05-31


### Bug Fixes

a90eb8 - YAML syntax - Vyacheslav Shimarulin

3f26cf - double release - Vyacheslav Shimarulin


- - -
## 1.0.7 - 2021-05-31


### Bug Fixes

4a1c3c - add personal access token - Vyacheslav Shimarulin


- - -
## 1.0.6 - 2021-05-31


### Bug Fixes

659e00 - only tags on deploy - Vyacheslav Shimarulin


- - -
## 1.0.5 - 2021-05-31


### Bug Fixes

9010b7 - get tags on deploy - Vyacheslav Shimarulin


- - -
## 1.0.4 - 2021-05-31


### Bug Fixes

519831 - workflow naming, ignore tags - Vyacheslav Shimarulin


- - -
## 1.0.3 - 2021-05-31


### Bug Fixes

f7212c - split release and deploy workflow - Vyacheslav Shimarulin


- - -
## 1.0.2 - 2021-05-29


### Bug Fixes

b70f17 - 'git push' on hook - Vyacheslav Shimarulin


- - -
## 1.0.1 - 2021-05-29


### Miscellaneous Chores

39df83 - test on master - Vyacheslav Shimarulin

f838bf - 1.0.1 - github-actions


### Continuous Integration

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


### Bug Fixes

3d9afc - cog settings - Vyacheslav Shimarulin

d40694 - git checkout main - Vyacheslav Shimarulin

7ae4c1 - revert changes from bump version 1.0.1 - Vyacheslav Shimarulin

00e932 - push changes to git - Vyacheslav Shimarulin

b664ea - test create release - Vyacheslav Shimarulin


- - -
## 1.0.0 - 2021-05-29


### Style

8b5891 - fmt - Vyacheslav Shimarulin

062d63 - select disk prompt struct - Vyacheslav Shimarulin

28a234 - reformat project with cargo - Vyacheslav Shimarulin


### Bug Fixes

5ab139 - command name - Vyacheslav Shimarulin

9f51b9 - file path - Vyacheslav Shimarulin

5739cc - write file - Vyacheslav Shimarulin

2f00f3 - hostnamectl output status - Vyacheslav Shimarulin

35ae65 - UEFI boot option - Vyacheslav Shimarulin

832215 - hostname prompt - Vyacheslav Shimarulin


### Documentation

2f0443 - description - Vyacheslav Shimarulin

c50933 - principles and features - Vyacheslav Shimarulin

ef921d - ssh connection to virtual host - Vyacheslav Shimarulin

e0f6b3 - workflow notes - Vyacheslav Shimarulin

3dc91d - add build notes to readme - Vyacheslav Shimarulin


### Miscellaneous Chores

82fcf9 - confirm message color - Vyacheslav Shimarulin


### Continuous Integration

0426e0 - move build to pre-bump hooks - Vyacheslav Shimarulin

7089b6 - add Cargo.lock to commit - Vyacheslav Shimarulin

93b9d6 - split post-bump command - Vyacheslav Shimarulin

3d1979 - create release - Vyacheslav Shimarulin

536caa - add cocogitto - Vyacheslav Shimarulin

836339 - add release action - Vyacheslav Shimarulin


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


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).