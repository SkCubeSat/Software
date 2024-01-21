/*
 * include.cpp  Created on: 29 Apr 2014
 * Copyright (c) 2014 Derek Molloy (www.derekmolloy.ie)
 * Made available for the book "Exploring BeagleBone"
 * If you use this code in your work please cite:
 *   Derek Molloy, "Exploring BeagleBone: Tools and Techniques for Building
 *   with Embedded Linux", Wiley, 2014, ISBN:9781118935125.
 * See: www.exploringbeaglebone.com
 * Licensed under the EUPL V.1.1
 *
 * This Software is provided to You under the terms of the European
 * Union Public License (the "EUPL") version 1.1 as published by the
 * European Union. Any use of this Software, other than as authorized
 * under this License is strictly prohibited (to the extent such use
 * is covered by a right of the copyright holder of this Software).
 *
 * This Software is provided under the License on an "AS IS" basis and
 * without warranties of any kind concerning the Software, including
 * without limitation merchantability, fitness for a particular purpose,
 * absence of defects or errors, accuracy, and non-infringement of
 * intellectual property rights other than copyright. This disclaimer
 * of warranty is an essential part of the License and a condition for
 * the grant of any rights to this Software.
 *
 * For more details, see http://www.derekmolloy.ie/
 */

#include "GPIO.h"


namespace PB
{

	namespace fs = std::filesystem;

	/**
	 *
	 * @param Number The include number for the BBB
	 */
    GPIO::GPIO(uint32_t Number) : m_Number(Number)
	{
		std::stringstream ss;
		ss << "/sys/class/gpio/" << "gpio" << m_Number;
		m_FilePath = ss.str();

       /* debounceTime = 0;
        togglePeriod=100;
        toggleNumber=-1; //infinite number
        callbackFunction = NULL;
        threadRunning = false;

        ostringstream s;
        s << "gpio" << number;
        name = string(s.str());
        path = GPIO_PATH + this->name + "/";
        exportGPIO();*/

	   // export include
		WriteFile("/sys/class/gpio/", "export", std::to_string(m_Number));
        // need to give Linux time to set up the sysfs structure
        usleep(250000); // 250ms delay
    }

	void GPIO::SetDirection(GPIO::Direction Direction)
	{
		if(Direction == Direction::eInput) { WriteFile(m_FilePath, "direction", "in"); }
		else { WriteFile(m_FilePath, "direction", "out"); }
	}

	void GPIO::SetValue(GPIO::Value Value)
	{
		if(Value == Value::eHigh) { WriteFile(m_FilePath, "value", "1"); }
		else { WriteFile(m_FilePath, "value", "0"); }
	}

/*
    int include::setEdgeType(GPIO_EDGE value){
        switch(value){
            case NONE: return this->write(this->path, "edge", "none");
                break;
            case RISING: return this->write(this->path, "edge", "rising");
                break;
            case FALLING: return this->write(this->path, "edge", "falling");
                break;
            case BOTH: return this->write(this->path, "edge", "both");
                break;
        }
        return -1;
    }

    int include::setActiveLow(bool isLow){
        if(isLow) return this->write(this->path, "active_low", "1");
        else return this->write(this->path, "active_low", "0");
    }

    int include::setActiveHigh(){
        return this->setActiveLow(false);
    }

    GPIO_VALUE include::getValue(){
        string input = this->read(this->path, "value");
        if (input == "0") return LOW;
        else return HIGH;
    }

    GPIO_DIRECTION include::getDirection(){
        string input = this->read(this->path, "direction");
        if (input == "in") return INPUT;
        else return OUTPUT;
    }

    GPIO_EDGE include::getEdgeType(){
        string input = this->read(this->path, "edge");
        if (input == "rising") return RISING;
        else if (input == "falling") return FALLING;
        else if (input == "both") return BOTH;
        else return NONE;
    }

    int include::streamOpen(){
        stream.open((path + "value").c_str());
        return 0;
    }
    int include::streamWrite(GPIO_VALUE value){
        stream << value << std::flush;
        return 0;
    }
    int include::streamClose(){
        stream.close();
        return 0;
    }

    int include::toggleOutput(){
        this->setDirection(OUTPUT);
        if ((bool) this->getValue()) this->setValue(LOW);
        else this->setValue(HIGH);
        return 0;
    }

    int include::toggleOutput(int time){ return this->toggleOutput(-1, time); }
    int include::toggleOutput(int numberOfTimes, int time){
        this->setDirection(OUTPUT);
        this->toggleNumber = numberOfTimes;
        this->togglePeriod = time;
        this->threadRunning = true;
        if(pthread_create(&this->thread, NULL, &threadedToggle, static_cast<void*>(this))){
            perror("include: Failed to create the toggle thread");
            this->threadRunning = false;
            return -1;
        }
        return 0;
    }

// This thread function is a friend function of the class
    void* threadedToggle(void *value){
        include *gpio = static_cast<include*>(value);
        bool isHigh = (bool) gpio->getValue(); //find current value
        while(gpio->threadRunning){
            if (isHigh)	gpio->setValue(HIGH);
            else gpio->setValue(LOW);
            usleep(gpio->togglePeriod * 500);
            isHigh=!isHigh;
            if(gpio->toggleNumber>0) gpio->toggleNumber--;
            if(gpio->toggleNumber==0) gpio->threadRunning=false;
        }
        return 0;
    }

// Blocking Poll - based on the epoll socket code in the epoll man page
    int include::waitForEdge(){
        this->setDirection(INPUT); // must be an input pin to poll its value
        int fd, i, epollfd, count=0;
        struct epoll_event ev;
        epollfd = epoll_create(1);
        if (epollfd == -1) {
            perror("include: Failed to create epollfd");
            return -1;
        }
        if ((fd = open((this->path + "value").c_str(), O_RDONLY | O_NONBLOCK)) == -1) {
            perror("include: Failed to open file");
            return -1;
        }

        //ev.events = read operation | edge triggered | urgent data
        ev.events = EPOLLIN | EPOLLET | EPOLLPRI;
        ev.data.fd = fd;  // attach the file file descriptor

        //Register the file descriptor on the epoll instance, see: man epoll_ctl
        if (epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &ev) == -1) {
            perror("include: Failed to add control interface");
            return -1;
        }
        while(count<=1){  // ignore the first trigger
            i = epoll_wait(epollfd, &ev, 1, -1);
            if (i==-1){
                perror("include: Poll Wait fail");
                count=5; // terminate loop
            }
            else {
                count++; // count the triggers up
            }
        }
        close(fd);
        if (count==5) return -1;
        return 0;
    }

// This thread function is a friend function of the class
    void* threadedPoll(void *value){
        include *gpio = static_cast<include*>(value);
        while(gpio->threadRunning){
            gpio->callbackFunction(gpio->waitForEdge());
            usleep(gpio->debounceTime * 1000);
        }
        return 0;
    }

    int include::waitForEdge(CallbackType callback){
        this->threadRunning = true;
        this->callbackFunction = callback;
        // create the thread, pass the reference, address of the function and data
        if(pthread_create(&this->thread, NULL, &threadedPoll, static_cast<void*>(this))){
            perror("include: Failed to create the poll thread");
            this->threadRunning = false;
            return -1;
        }
        return 0;
    }*/

    GPIO::~GPIO()
	{
		// unexport
	    WriteFile("/sys/class/gpio/", "unexport", std::to_string(m_Number));
    }

	GPIO::Direction GPIO::GetDirection()
	{
		if(ReadFile(m_FilePath, "direction") == "in") { return Direction::eInput; }
		else { return Direction::eOutput; }
	}

	GPIO::Value GPIO::GetValue()
	{
		if(ReadFile(m_FilePath, "value") == "1") { return Value::eHigh; }
		else { return Value::eLow; }
	}

	void GPIO::Write(std::string FileName, std::string Value)
	{
		WriteFile(m_FilePath, std::move(FileName), std::move(Value));
	}

	std::string GPIO::Read(std::string FileName)
	{
		return ReadFile(m_FilePath, std::move(FileName));
	}
}
