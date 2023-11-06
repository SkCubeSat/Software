# Setup STM32 IDE for coding @ home

## STM32 software development

##### 1. First head over to [STM32Cube IDE](https://www.st.com/en/development-tools/stm32cubeide.html) and download and install the latest version of the STM32cube IDE.
- Note that you can use any name or anything in the input field as long as its a valid email address since the software download link will go to your email inbox
- Use a [temporary email for sign up if you'd like](https://temp-mail.org/en/)
- 

![](https://i.imgur.com/HmqLSrb.gif)



##### 2. Install the software but DO NO CREATE A NEW PROJECT
- We will be using git to pull the current working repository from [Github](https://github.com) instead
- **if you are on windows** I suggest first installing a couple things to make your life easier.
	- get the latest windows [PowerShell 7](https://github.com/PowerShell/PowerShell/releases/tag/v7.3.4)
	- install [git](https://git-scm.com/downloads) (make sure you select to get the credentials manager it will be useful to login to your GitHub so you can push changes)
	- install a package manager to get git and a variety of other useful tools (but you do need git!) like [chocolatey](https://chocolatey.org)

##### 3. Git clone the project and start development!
- run `git clone https://github.com/USST-CUBICS/Software` in your terminal in the folder of your choice. (you will be working from this folder)
- then open up STM32CubeIDE and open the project folder of your choice!
- ![[OpenProject.gif]]

#### 4. Making changes (commit and push to GitHub)
- To push the changes you make to the code. Make sure you have git installed and open a PowerShell window (or terminal window in MAC) to the "software" downloads folder. Mine is conveniently placed in `~\Documents\RADSAT-SK2\software` but yours may be different.
- Once you're there you should be able to run the command: `git status`  and it should output the following message:
- ![[git status.gif]]
- Make your desired changes and then once you are ready to commit. do `git status` again to check the changes that you made and then `git commit -a -m "<your message>"` to commit the changes. Finally once you are ready to send it to GitHub do a `git push`
- There you go you just made a change to the repository!