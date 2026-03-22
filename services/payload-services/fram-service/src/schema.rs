use async_graphql::{Object, SimpleObject, Schema};


struct Query;
struct Mutation;

#[derive(SimpleObject, Clone)]
struct Data{
    Component: enum
    Status: bool
    Confidence: u32
}

#[derive(SimpleObject, clone)]
struct Error{
    Component: enum
    failure: str  
}

enum Checks{
    SolarPanels,
    antenna,
}

#[Object]
impl Query{

    async fn GetStatus(&self, Part: Checks) -> Result<Data>{
        let Part_Address = getaddresses(Checks)             
        
        Ok(Data {
            Component: Checks,
            Status: Part_Status
            Confidence: 
        })

        Err(Error {
            Component: Checks
            failure: "Couldnt get address of Component"
        })

    }   
}


#[Object]
impl Mutation{


    async fn SetStatus(&self, Part: Checks){
        

    }




