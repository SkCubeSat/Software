use async_graphql::{Object, SimpleObject, Schema};


struct Query;
struct Mutation;

#[derive(SimpleObject, Clone)]
struct Data{
    Component: enum
    Status: ____
    Confidence: u32
}

enum Checks{
    SolarPanels,
    antenna,
}

#[Object]
impl Query{

    async fn GetStatus(&self, Part: Checks) -> Result<Data>{
        let Part_Status = ____
        
        Ok(Data {
            Component: Checks,
            Status: Part_Status
            Confidence: ____
        }

    {
}


#[Object]
impl Mutation{


    async fn SetStatus(&self, Part: Checks){
        

    }




