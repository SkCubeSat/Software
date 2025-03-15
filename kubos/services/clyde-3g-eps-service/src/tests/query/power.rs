//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::*;

#[test]
fn get_power_good() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{
            power {
                daughterboard,
                motherboard
            }
        }"#;

    let expected = json!({
        "power": {
            "daughterboard": "ON",
            "motherboard": "ON"
        }
    });

    test!(service, query, expected);
}

#[test]
fn get_power_bad() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{
            power {
                daughterboard,
                motherboard
            }
        }"#;

    let expected = json!({
        "power": {
            "daughterboard": "OFF",
            "motherboard": "OFF"
        }
    });

    test!(service, query, expected);
}
