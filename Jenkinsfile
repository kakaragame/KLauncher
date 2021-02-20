pipeline {
    agent any
    stages {
        stage ('Build') {
            steps {
             sh '''export PATH="$HOME/.cargo/bin:$PATH"

             yarn tauri build'''
            }
            post {
                success {
                  archiveArtifacts artifacts: 'src-tauri/target/release/bundle/**/*', followSymlinks: false
                }
            }
        }
    }
}