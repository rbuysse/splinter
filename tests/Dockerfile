# Copyright 2018-2021 Cargill Incorporated
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM splintercommunity/splinter-dev:v8

RUN apt-get update \
 && apt-get install -y \
 postgresql-client

COPY Cargo.toml /build/Cargo.toml
COPY libsplinter /build/libsplinter
COPY splinterd /build/splinterd
COPY cli /build/cli
COPY services/scabbard/cli /build/services/scabbard/cli
COPY services/scabbard/libscabbard /build/services/scabbard/libscabbard
COPY services/health /build/services/health
COPY examples/gameroom/database /build/examples/gameroom/database
COPY examples/gameroom/daemon /build/examples/gameroom/daemon
COPY examples/gameroom/cli /build/examples/gameroom/cli

COPY justfile /build

WORKDIR /build
