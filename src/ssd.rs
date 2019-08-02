use yew::prelude::*;
use crate::ships::System;

#[derive(Clone)]
pub struct Icon{
    system:System
}

#[derive(Clone,PartialEq)]
pub struct IconProps{
    pub system:System
}

impl Default for IconProps{
    fn default()->Self{
        IconProps{
            system:System::cannon("Beam",1,crate::ships::Arcs::all())
        }
    }
}

impl Component for Icon{
    type Properties = IconProps;
    type Message = ();

    fn create(props:Self::Properties,_:ComponentLink<Self>)->Self{
        Icon{system:props.system}
    }

    fn update(&mut self,_:Self::Message)->ShouldRender{
        false
    }
}


impl Renderable<Self> for Icon{
    fn view(&self)->Html<Self>{
        html!{
            <div class="icon-temp"></div>
        }
    }
}

    