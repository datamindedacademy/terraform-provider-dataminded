FROM gitpod/workspace-full 

# install Terraform
RUN sudo apt-get update && sudo apt-get install -y gnupg software-properties-common

RUN wget -O- https://apt.releases.hashicorp.com/gpg | \
    gpg --dearmor | \
    sudo tee /usr/share/keyrings/hashicorp-archive-keyring.gpg > /dev/null

RUN echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] \
    https://apt.releases.hashicorp.com $(lsb_release -cs) main" | \
    sudo tee /etc/apt/sources.list.d/hashicorp.list

RUN apt update

RUN apt-get install terraform

COPY . /home/gitpod

