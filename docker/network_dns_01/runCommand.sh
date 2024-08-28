#/bin/env bash


networkName="my_privateNetwork_${RANDOM:0:3}"


echo "# 建立私有網路"
cmdList=(docker network create $networkName)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 列出網路"
cmdList=(docker network ls)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 運行 c01 容器"
cmdList=(docker run --rm -dt --name c01 --network $networkName alpine sh)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 運行 link c01 的 c02 容器"
cmdList=(docker run --rm -dt --name c02 --network $networkName --link c01:c1 alpine sh)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 列出容器的網路"
cmdList=(docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Networks}}")
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e '\n\n# 顯示 c02 容器的 "/etc/hosts" 文件'
cmdList=(docker exec c02 cat /etc/hosts)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 從 c02 容器 ping c01 容器"
cmdList=(docker exec c02 ping c1 -c 2)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 停止 c01, c02 容器"
cmdList=(docker stop c01 c02)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"


echo -e "\n\n# 刪除私有網路"
cmdList=(docker network rm $networkName)
echo "\$ ${cmdList[*]}"
"${cmdList[@]}"
