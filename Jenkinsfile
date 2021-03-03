#!groovy

// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------

pipeline {
    agent {
        node {
            label 'master'
            customWorkspace "workspace/${env.BUILD_TAG}"
        }
    }

    triggers {
        cron(env.BRANCH_NAME == 'main' ? 'H 2 * * *' : '')
    }

    options {
        timestamps()
        buildDiscarder(logRotator(daysToKeepStr: '31'))
    }

    environment {
        ISOLATION_ID = sh(returnStdout: true, script: 'printf $BUILD_TAG | sha256sum | cut -c1-64').trim()
        COMPOSE_PROJECT_NAME = sh(returnStdout: true, script: "printf $BUILD_TAG | sha256sum | cut -c1-64").trim()
        VERSION="AUTO_STRICT"
        BUILD_MODE="--verbose --release"
    }

    stages {
        // stage('Check Whitelist') {
        //     steps {
        //         readTrusted 'bin/whitelist'
        //         sh './bin/whitelist "$CHANGE_AUTHOR" /etc/jenkins-authorized-builders'
        //     }
        //     when {
        //         not {
        //             branch 'main'
        //         }
        //     }
        // }

        stage("Lint gameroom client") {
            steps {
                sh 'just ci-lint-client'
            }
        }

        stage("Run Lint/Clippy on Splinter") {
            steps {
                sh 'just ci-lint-splinter'
            }
        }

        stage("Test Splinter") {
            steps {
                sh 'just ci-test'
            }
        }

        stage("Test Gameroom") {
            steps {
                sh 'just ci-test-gameroom'
            }
        }

        stage("Test Gameroom UI") {
            steps {
                sh 'docker-compose -f examples/gameroom/tests/cypress/docker-compose.yaml up --abort-on-container-exit'
            }
        }


    }
    post {
        always {
            sh 'docker-compose -f docker/compose/run-lint.yaml down'
            sh 'docker-compose -f tests/test-splinter.yaml down'
            sh 'docker-compose -f examples/gameroom/tests/cypress/docker-compose.yaml down'
            sh 'docker-compose -f examples/gameroom/tests/docker-compose.yaml down'
        }
    }
}
