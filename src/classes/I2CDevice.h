class I2CDevice {
private:
    unsigned int bus, device;
    int file;
public:
    I2CDevice(unsigned int bus, unsigned int device);
    virtual int open();
    virtual int write(unsigned char value);
    virtual unsigned char readRegister(unsigned int registerAddress);
    virtual unsigned char* readRegisters(unsigned int number,
                                        unsigned int fromAddress=0);
    virtual int writeRegister(unsigned int registerAddress, unsigned
                            char value);
    virtual void debugDumpRegisters(unsigned int number);
    virtual void close();
    virtual ~I2CDevice();
};


// write own thing

//write my own thing oren

//efvefgvergergerg

// Test