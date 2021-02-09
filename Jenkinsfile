pipeline {
    agent any
    stages {
        stage ('Build') {
            steps {
             sh '''export PATH="$HOME/.cargo/bin:$PATH"
             export DISCORD_KEY=${DISCORD_KEY}

             cargo build
             cargo build --target x86_64-pc-windows-gnu'''
            }
            post {
                success {
archiveArtifacts artifacts: 'target/debug/klauncher, target/x86_64-pc-windows-gnu/debug/klauncher.exe', followSymlinks: false
                }
            }
        }
    }
}